# Command-Line Help for `configuratarr`

This document contains the help content for the `configuratarr` command-line program.

**Command Overview:**

* [`configuratarr`↴](#configuratarr)
* [`configuratarr plan`↴](#configuratarr-plan)
* [`configuratarr apply`↴](#configuratarr-apply)

## `configuratarr`

Declarative config management for media services

**Usage:** `configuratarr [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `plan` — Show planned changes without applying them
* `apply` — Apply planned changes to services

###### **Options:**

* `-c`, `--config <CONFIG>` — Path to config file

  Default value: `configuratarr.yaml`
* `--wait-for-healthy` — Wait for each service's health endpoint to respond before planning or applying (skipped for services that declare no health check)
* `--wait-timeout <WAIT_TIMEOUT>` — Max seconds to wait per service when --wait-for-healthy is set

  Default value: `120`



## `configuratarr plan`

Show planned changes without applying them

**Usage:** `configuratarr plan [OPTIONS]`

###### **Options:**

* `--prune` — Also plan deletion of resources not present in config



## `configuratarr apply`

Apply planned changes to services

**Usage:** `configuratarr apply [OPTIONS]`

###### **Options:**

* `--prune` — Also delete resources not present in config
* `--auto-approve` — Skip the interactive confirmation prompt and apply immediately



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
