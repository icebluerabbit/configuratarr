# Local e2e dev shell for bindery-v1.
#
# Bindery reads its instance API key from `BINDERY_API_KEY`, so there is no
# onboarding to drive: start it in a temp data dir with a known key, export
# BINDERY_URL + BINDERY_API_KEY, clean up on exit.
{
  pkgs,
  e2eShell,
  common,
}:
let
  bindery = pkgs.callPackage ../pkgs/bindery.nix { };
in
pkgs.mkShell {
  inputsFrom = [ e2eShell ];
  packages = [
    bindery
    pkgs.curl
  ];
  shellHook = ''
    echo "=== Configuratarr E2E DevShell (bindery-v1) ==="
    ${common}

    _BD_DATA=$(mktemp -d -t configuratarr-bindery-XXXXXX)
    _BD_URL="http://localhost:8787"
    _BD_KEY="configuratarre2econfiguratarre2e"

    # Bindery takes everything from the environment, so its temp dir never
    # reaches the command line — pass it explicitly to the port reclaimer.
    if ! e2e_reclaim_port 8787 Bindery "*$_BD_DATA*"; then
      return 2>/dev/null || exit 1
    fi

    echo "  starting Bindery..."
    BINDERY_PORT=8787 \
    BINDERY_DB_PATH="$_BD_DATA/bindery.db" \
    BINDERY_DATA_DIR="$_BD_DATA" \
    BINDERY_LIBRARY_DIR="$_BD_DATA/books" \
    BINDERY_DOWNLOAD_DIR="$_BD_DATA/downloads" \
    BINDERY_API_KEY="$_BD_KEY" \
      ${pkgs.lib.getExe bindery} > "$_BD_DATA/bindery.log" 2>&1 &
    _BD_PID=$!

    _bd_wait() {
      local i=0
      while [ $i -lt 90 ]; do
        curl -sf "$_BD_URL/api/v1/system/status" -H "X-Api-Key: $_BD_KEY" > /dev/null 2>&1 \
          && return 0
        sleep 1; i=$((i + 1))
      done
      return 1
    }

    if _bd_wait; then
      export BINDERY_URL="$_BD_URL"
      export BINDERY_API_KEY="$_BD_KEY"
      echo "  Bindery ready — $BINDERY_URL"
      echo ""
      echo "  cargo nextest run -p bindery-v1 --run-ignored all -j1"
    else
      echo "  Bindery failed to start — check $_BD_DATA/bindery.log"
      kill "$_BD_PID" 2>/dev/null
    fi

    _bd_cleanup() {
      kill "$_BD_PID" 2>/dev/null
      wait "$_BD_PID" 2>/dev/null
      rm -rf "$_BD_DATA"
    }
    trap _bd_cleanup EXIT
  '';
}
