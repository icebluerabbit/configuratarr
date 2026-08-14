# Local e2e dev shell for lazylibrarian-v1.
# LazyLibrarian is not in nixpkgs; the package comes from lazylibrarian-flake.
{
  pkgs,
  e2eShell,
  common,
  lazylibrarian,
}:
let
  apiKey = "configuratarre2e0000000000000000"; # exactly 32 chars
in
pkgs.mkShell {
  inputsFrom = [ e2eShell ];
  packages = [
    lazylibrarian
    pkgs.curl
  ];
  shellHook = ''
    echo "=== Configuratarr E2E DevShell (lazylibrarian-v1) ==="
    ${common}

    _LL_DATA=$(mktemp -d -t configuratarr-lazylibrarian-XXXXXX)
    cat > "$_LL_DATA/config.ini" <<'EOF'
    [General]
    http_host = 127.0.0.1
    http_port = 5299
    http_root = /
    launch_browser = 0
    api_enabled = 1
    api_key = configuratarre2e0000000000000000
    EOF

    if ! e2e_reclaim_port 5299 LazyLibrarian; then
      return 2>/dev/null || exit 1
    fi

    echo "  starting lazylibrarian..."
    # LazyLibrarian self-restarts once on first run (the launcher normally
    # re-execs; run directly it just exits), so keep relaunching it.
    #
    # The whole subshell is redirected, not just the server: unlike the sibling
    # shells (whose `$!` *is* the server), the relaunch loop leaves both itself
    # and its `lazylibrarian` child holding this shell's stdout. Under
    # `nix develop … --command cargo … | somepipe` a leaked one keeps the pipe's
    # write end open after `nix develop` exits, so the reader never sees EOF and
    # the whole invocation hangs long after the tests passed.
    #
    # The loop also stops relaunching once this shell is gone. The EXIT trap below
    # does not reliably fire under `--command` (see _common.nix), so the loop can
    # outlive the run — and then the *next* run's `e2e_reclaim_port` would kill the
    # stale server only for the orphan to relaunch it onto the port the fresh one
    # is about to bind. Checking the owning shell each round makes the orphan exit
    # the moment its server is reclaimed instead.
    _LL_SHELL=$$
    ( for _n in 1 2 3 4 5; do
        kill -0 "$_LL_SHELL" 2>/dev/null || break
        lazylibrarian --datadir "$_LL_DATA" --config "$_LL_DATA/config.ini" \
          --port 5299 --nolaunch
        sleep 1
      done ) >> "$_LL_DATA/lazylibrarian.log" 2>&1 &
    _LL_PID=$!

    _ll_wait() {
      local i=0
      while [ $i -lt 90 ]; do
        curl -sf "http://localhost:5299/api?cmd=getVersion&apikey=${apiKey}" > /dev/null 2>&1 && return 0
        sleep 1; i=$((i + 1))
      done
      return 1
    }

    if _ll_wait; then
      export LAZYLIBRARIAN_URL="http://localhost:5299"
      export LAZYLIBRARIAN_API_KEY="${apiKey}"
      echo "  lazylibrarian ready — $LAZYLIBRARIAN_URL"
      echo ""
      echo "  cargo nextest run -p lazylibrarian-v1 --test e2e --run-ignored all -j1"
    else
      echo "  lazylibrarian failed to start — check $_LL_DATA/lazylibrarian.log"
      kill "$_LL_PID" 2>/dev/null
    fi

    _ll_cleanup() {
      # Kill the relaunch loop first, or it just starts another server.
      kill "$_LL_PID" 2>/dev/null
      wait "$_LL_PID" 2>/dev/null
      # $_LL_PID is the loop, not the server — the running `lazylibrarian` is its
      # child and outlives it, so stop that too before the datadir disappears
      # under it. `e2e_reclaim_port` only kills a `configuratarr-`-marked process
      # (our temp datadir is on its cmdline) and reports anything else.
      e2e_reclaim_port 5299 LazyLibrarian > /dev/null 2>&1 || true
      rm -rf "$_LL_DATA"
    }
    trap _ll_cleanup EXIT
  '';
}
