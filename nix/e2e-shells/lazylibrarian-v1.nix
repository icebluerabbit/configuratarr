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
    ( for _n in 1 2 3 4 5; do
        lazylibrarian --datadir "$_LL_DATA" --config "$_LL_DATA/config.ini" \
          --port 5299 --nolaunch >> "$_LL_DATA/lazylibrarian.log" 2>&1
        sleep 1
      done ) &
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
      kill "$_LL_PID" 2>/dev/null
      wait "$_LL_PID" 2>/dev/null
      rm -rf "$_LL_DATA"
    }
    trap _ll_cleanup EXIT
  '';
}
