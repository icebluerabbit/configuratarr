use std::io::Write;

use anyhow::Result;
use clap::Parser;
use core_cli::{Cli, Command};
use core_config::{ConfigFile, Instance, Options, load};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let cfg = load(&cli.config)?;

    if cli.wait_for_healthy {
        wait_all(&cfg, std::time::Duration::from_secs(cli.wait_timeout)).await?;
    }

    match cli.command {
        Command::Plan { prune } => plan(&cfg, Options { prune }).await,
        Command::Apply {
            prune,
            auto_approve,
        } => apply(&cfg, Options { prune }, auto_approve).await,
    }
}

/// Wait for every instance that declares a health check to become ready, before
/// any planning or applying. Instances without one pass through silently.
async fn wait_all(cfg: &ConfigFile, timeout: std::time::Duration) -> Result<()> {
    for label in sorted_labels(cfg) {
        let inst = &cfg[label];
        if inst.health_check().is_none() {
            continue;
        }
        print!("waiting for {label} ({})… ", inst.service.type_name());
        std::io::stdout().flush()?;
        inst.wait_healthy(timeout)
            .await
            .map_err(|e| anyhow::anyhow!("{label}: {e:#}"))?;
        println!("healthy");
    }
    Ok(())
}

/// Instance labels in deterministic order (each instance is independent).
fn sorted_labels(cfg: &ConfigFile) -> Vec<&String> {
    let mut labels: Vec<&String> = cfg.keys().collect();
    labels.sort();
    labels
}

/// Print the plan header for one instance.
fn header(label: &str, inst: &Instance) {
    println!("══ {label} ({}) ══", inst.service.type_name());
}

/// `plan` command: render every instance's planned changes; write nothing.
async fn plan(cfg: &ConfigFile, opts: Options) -> Result<()> {
    for label in sorted_labels(cfg) {
        let inst = &cfg[label];
        let plan = inst
            .plan(opts)
            .await
            .map_err(|e| anyhow::anyhow!("plan `{label}`: {e:#}"))?;
        header(label, inst);
        if plan.is_empty() {
            println!("  no changes");
        } else {
            print!("{}", plan.render());
        }
        println!();
    }
    Ok(())
}

/// `apply` command: plan all instances, render, confirm (unless `--auto-approve`
/// or nothing would change), then apply.
async fn apply(cfg: &ConfigFile, opts: Options, auto_approve: bool) -> Result<()> {
    let labels = sorted_labels(cfg);

    // Phase 1 — plan every instance and show the combined diff.
    let mut plans = Vec::with_capacity(labels.len());
    for label in &labels {
        let inst = &cfg[*label];
        let plan = inst
            .plan(opts)
            .await
            .map_err(|e| anyhow::anyhow!("plan `{label}`: {e:#}"))?;
        header(label, inst);
        if plan.is_empty() {
            println!("  no changes");
        } else {
            print!("{}", plan.render());
        }
        println!();
        plans.push(plan);
    }

    if plans.iter().all(|p| p.is_empty()) {
        println!("No changes. Nothing to apply.");
        return Ok(());
    }

    // Phase 2 — confirm before writing anything.
    if !auto_approve && !confirm()? {
        println!("Aborted. No changes applied.");
        return Ok(());
    }

    // Phase 3 — apply. Re-runs the graph (server-assigned ids resolve live).
    for label in &labels {
        let inst = &cfg[*label];
        let r = inst
            .apply(opts)
            .await
            .map_err(|e| anyhow::anyhow!("apply `{label}`: {e:#}"))?;
        println!(
            "{label} ({}): created {}, updated {}, deleted {}, unchanged {}",
            inst.service.type_name(),
            r.created,
            r.updated,
            r.deleted,
            r.unchanged,
        );
    }
    Ok(())
}

/// Prompt on the terminal and read a yes/no answer. Defaults to no.
fn confirm() -> Result<bool> {
    print!("Apply these changes? [y/N] ");
    std::io::stdout().flush()?;
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    let answer = line.trim().to_ascii_lowercase();
    Ok(answer == "y" || answer == "yes")
}
