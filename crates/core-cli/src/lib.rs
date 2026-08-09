use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "configuratarr",
    about = "Declarative config management for media services"
)]
pub struct Cli {
    /// Path to config file
    #[arg(
        short,
        long,
        default_value = "configuratarr.yaml",
        env = "CONFIGURATARR_CONFIG"
    )]
    pub config: PathBuf,

    /// Wait for each service's health endpoint to respond before planning or
    /// applying (skipped for services that declare no health check)
    #[arg(long)]
    pub wait_for_healthy: bool,

    /// Max seconds to wait per service when --wait-for-healthy is set
    #[arg(long, default_value_t = 120)]
    pub wait_timeout: u64,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Show planned changes without applying them
    Plan {
        /// Also plan deletion of resources not present in config
        #[arg(long)]
        prune: bool,
    },
    /// Apply planned changes to services
    Apply {
        /// Also delete resources not present in config
        #[arg(long)]
        prune: bool,
        /// Skip the interactive confirmation prompt and apply immediately
        #[arg(long)]
        auto_approve: bool,
    },
}
