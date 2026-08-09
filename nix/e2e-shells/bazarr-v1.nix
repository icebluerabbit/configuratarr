# Local e2e dev shell for bazarr-v1.
# Starts Bazarr in a temp data dir, waits for it to generate its API key, reads
# the key out of the config file, exports BAZARR_URL + BAZARR_API_KEY, and
# cleans up on exit.
{
  pkgs,
  e2eShell,
  common,
}:
pkgs.mkShell {
  inputsFrom = [ e2eShell ];
  packages = [ pkgs.bazarr ];
  shellHook = ''
    echo "=== Configuratarr E2E DevShell (bazarr-v1) ==="
    ${common}

    _BZ_DATA=$(mktemp -d -t configuratarr-bazarr-XXXXXX)

    if ! e2e_reclaim_port 6767 Bazarr; then
      return 2>/dev/null || exit 1
    fi

    echo "  starting Bazarr..."
    bazarr --no-update --config "$_BZ_DATA" > "$_BZ_DATA/bazarr.log" 2>&1 &
    _BZ_PID=$!

    _bz_key() {
      # Only one of config.yaml/config.ini exists; grep exits 2 on the absent
      # one. Swallow it with `|| true` so the pipeline stays clean under pipefail.
      { grep -hoE -m1 'apikey[:=] *[A-Za-z0-9]+' \
        "$_BZ_DATA/config/config.yaml" "$_BZ_DATA/config/config.ini" 2>/dev/null \
        || true; } | grep -oE '[A-Za-z0-9]+$'
    }

    _bz_wait() {
      local i=0
      while [ $i -lt 90 ]; do
        local k; k=$(_bz_key)
        if [ -n "$k" ] && curl -sf "http://localhost:6767/api/system/status" \
             -H "X-API-KEY: $k" > /dev/null 2>&1; then
          return 0
        fi
        sleep 1; i=$((i + 1))
      done
      return 1
    }

    if _bz_wait && e2e_require "BAZARR_API_KEY" "$(_bz_key)"; then
      export BAZARR_URL="http://localhost:6767"
      export BAZARR_API_KEY="$(_bz_key)"
      echo "  Bazarr ready — $BAZARR_URL"
      echo ""
      echo "  cargo nextest run -p bazarr-v1 --run-ignored all -j1"
    else
      echo "  Bazarr failed to start — check $_BZ_DATA/bazarr.log"
      kill "$_BZ_PID" 2>/dev/null
    fi

    _bz_cleanup() {
      kill "$_BZ_PID" 2>/dev/null
      wait "$_BZ_PID" 2>/dev/null
      rm -rf "$_BZ_DATA"
    }
    trap _bz_cleanup EXIT
  '';
}
