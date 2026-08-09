# Local e2e dev shell for cleanuparr-v1.
# Starts Cleanuparr in a temp data dir, runs first-run setup, logs in for a JWT,
# reads the minted API key, exports CLEANUPARR_URL + CLEANUPARR_API_KEY, and
# cleans up on exit.
#
# Cleanuparr has no config-file API key (unlike bazarr): the key is generated
# with the admin account during setup and is only readable through the API after
# authenticating. So the bootstrap is four calls — create account, complete
# setup, log in, read the key — mirroring the autobrr shell.
#
# The setup-guard middleware makes ordering load-bearing: until
# `/api/auth/setup/complete` succeeds, every non-auth `/api/*` path answers 403
# `{"error":"Setup required"}`, which would otherwise surface to the suite as a
# baffling permissions failure.
{
  pkgs,
  e2eShell,
  common,
  cleanuparr,
}:
pkgs.mkShell {
  inputsFrom = [ e2eShell ];
  packages = [
    cleanuparr
    pkgs.curl
    pkgs.jq
  ];
  shellHook = ''
    echo "=== Configuratarr E2E DevShell (cleanuparr-v1) ==="
    ${common}

    _CP_DATA=$(mktemp -d -t configuratarr-cleanuparr-XXXXXX)

    # Cleanuparr takes its data dir from the environment, not argv, so a leaked
    # instance's cmdline carries no `configuratarr-` marker — hand the reclaimer
    # this package's store path so it still recognises the process as ours.
    if ! e2e_reclaim_port 11011 cleanuparr '${cleanuparr}/*'; then
      return 2>/dev/null || exit 1
    fi

    echo "  starting cleanuparr..."
    # CLEANUPARR_CONFIG_PATH keeps the SQLite db + config out of the store path.
    CLEANUPARR_CONFIG_PATH="$_CP_DATA" \
    CLEANUPARR_LOGS_PATH="$_CP_DATA/logs" \
    PORT=11011 \
    BIND_ADDRESS=127.0.0.1 \
      ${pkgs.lib.getExe cleanuparr} > "$_CP_DATA/cleanuparr.log" 2>&1 &
    _CP_PID=$!

    _cp_wait() {
      local i=0
      while [ $i -lt 90 ]; do
        curl -sf http://localhost:11011/health > /dev/null 2>&1 && return 0
        sleep 1; i=$((i + 1))
      done
      return 1
    }

    if _cp_wait; then
      # 1. create the admin account, 2. finish setup (this lifts the setup
      # guard on every other /api path), 3. log in, 4. read the API key.
      curl -sf -X POST http://localhost:11011/api/auth/setup/account \
        -H 'Content-Type: application/json' \
        -d '{"username":"admin","password":"configuratarre2e"}' > /dev/null
      curl -sf -X POST http://localhost:11011/api/auth/setup/complete > /dev/null
      _CP_JWT=$(curl -sf -X POST http://localhost:11011/api/auth/login \
        -H 'Content-Type: application/json' \
        -d '{"username":"admin","password":"configuratarre2e"}' \
        | jq -r '.tokens.accessToken // empty' | head -n1)
      _CP_KEY=$(curl -sf http://localhost:11011/api/account/api-key \
        -H "Authorization: Bearer $_CP_JWT" \
        | jq -r '.apiKey // empty' | head -n1)

      # Each bootstrap call fails quietly under `-sf … > /dev/null`; an empty key
      # would then reach the suite as a puzzling 401 rather than a setup error.
      if e2e_require "CLEANUPARR_API_KEY" "$_CP_KEY"; then
        export CLEANUPARR_URL="http://localhost:11011"
        export CLEANUPARR_API_KEY="$_CP_KEY"
        echo "  cleanuparr ready — $CLEANUPARR_URL"
        echo ""
        echo "  cargo nextest run -p cleanuparr-v1 --test e2e --run-ignored all -j1"
      else
        echo "  cleanuparr setup failed — check $_CP_DATA/cleanuparr.log"
        kill "$_CP_PID" 2>/dev/null
      fi
    else
      echo "  cleanuparr failed to start — check $_CP_DATA/cleanuparr.log"
      kill "$_CP_PID" 2>/dev/null
    fi

    trap '
      kill "$_CP_PID" 2>/dev/null
      rm -rf "$_CP_DATA"
    ' EXIT
  '';
}
