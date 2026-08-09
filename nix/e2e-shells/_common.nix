# Shell helpers shared by every e2e dev shell.
#
# Two failure modes these exist to kill:
#
# 1. **Leaked instances.** Each shell traps EXIT to stop its service, but under
#    `nix develop .#e2e-<svc> --command …` the trap does not reliably fire, so
#    the service outlives the run and keeps holding its port. The next run's
#    service then dies at bind time ("Aborted (core dumped)") while the health
#    probe happily answers from the *stale* instance — so the suite silently
#    tests a server nobody configured, and results swing between runs depending
#    on what was left behind. `e2e_reclaim_port` makes each run start from a
#    known-clean port.
#
# 2. **Silent onboarding failures.** The hooks drive setup wizards with
#    `curl -sf … > /dev/null`; when a step fails the pipeline still "succeeds"
#    and the exported key ends up empty. An empty key is not obviously broken —
#    some endpoints answer unauthenticated — so the shell announces "ready" and
#    the suite fails later with a confusing 401. `e2e_require` and
#    `e2e_retry` turn those into loud, immediate failures.
{ pkgs }:
''
  # Kill a leaked *configuratarr* e2e instance still holding $1, then wait for
  # the port to free. Only ever kills a process whose command line carries the
  # `configuratarr-` temp-datadir marker this repo's shells use — anything else
  # on the port is reported and left alone (it is not ours to kill).
  #
  # $3 is an optional extra glob to recognise as ours. Most shells pass their
  # temp data dir as a command-line argument, so the marker is already there;
  # a service configured purely through the environment (Cleanuparr, whose data
  # dir arrives as `CLEANUPARR_CONFIG_PATH`) has nothing but its store path on
  # the cmdline, so it passes that path here instead.
  e2e_reclaim_port() {
    local port="$1" label="$2" extra="''${3:-}" pids pid cmd i signal

    # TERM first, then KILL: some services (Bazarr, which forks workers) don't
    # exit on TERM, and a half-dead process still holding the port is exactly the
    # state this helper exists to prevent. Re-listing each round also catches a
    # child that inherited the socket from the parent we just killed.
    for signal in TERM KILL; do
      pids=$(${pkgs.lsof}/bin/lsof -t -i ":$port" -sTCP:LISTEN 2>/dev/null || true)
      [ -z "$pids" ] && return 0

      for pid in $pids; do
        cmd=$(tr '\0' ' ' < "/proc/$pid/cmdline" 2>/dev/null || true)
        if [ -n "$extra" ]; then
          case "$cmd" in
            $extra)
              echo "  reclaiming port $port from leaked $label (pid $pid, SIG$signal)"
              kill "-$signal" "$pid" 2>/dev/null || true
              continue
              ;;
          esac
        fi
        case "$cmd" in
          *configuratarr-*)
            echo "  reclaiming port $port from leaked $label (pid $pid, SIG$signal)"
            kill "-$signal" "$pid" 2>/dev/null || true
            ;;
          *)
            echo "  ERROR: port $port is held by a process this shell did not start:"
            echo "    pid $pid: $cmd"
            echo "  Stop it (or free the port) and re-enter the shell."
            return 1
            ;;
        esac
      done

      for i in $(seq 1 15); do
        ${pkgs.lsof}/bin/lsof -t -i ":$port" -sTCP:LISTEN > /dev/null 2>&1 || return 0
        sleep 1
      done
    done

    echo "  ERROR: port $port still busy after SIGKILLing the leaked $label."
    return 1
  }

  # Abort when a value the rest of the hook depends on came back empty. Onboarding
  # steps fail quietly, and an empty key reads as "ready" against endpoints that
  # do not require auth — so check it here rather than debugging a later 401.
  e2e_require() {
    local name="$1" value="$2"
    if [ -z "$value" ]; then
      echo "  ERROR: $name is empty — onboarding did not complete."
      return 1
    fi
    return 0
  }

  # Run a command until it succeeds, up to $1 attempts, 1s apart. Setup-wizard
  # endpoints answer 404/503 for a moment after the health endpoint goes green,
  # so a single attempt is a race even when the server is technically up.
  e2e_retry() {
    local attempts="$1" i; shift
    for i in $(seq 1 "$attempts"); do
      "$@" && return 0
      sleep 1
    done
    echo "  ERROR: command never succeeded after ''${attempts}s: $*"
    return 1
  }
''
