# Local e2e dev shell for autobrr-v1.
# Starts autobrr in a temp data dir, onboards the first user, logs in for a
# session, mints an API key via the API, exports AUTOBRR_URL + AUTOBRR_API_KEY,
# and cleans up on exit.
#
# autobrr has no key in a config file (unlike bazarr) — keys are minted through
# the API after onboarding. The mint returns exactly one key on a fresh (temp)
# data dir; `head -n1` guards against ever handing a multi-line value to an HTTP
# header (the trap that bit jellyfin-v11).
{
  pkgs,
  e2eShell,
  common,
}:
pkgs.mkShell {
  inputsFrom = [ e2eShell ];
  packages = [
    pkgs.autobrr
    pkgs.curl
    pkgs.jq
  ];
  shellHook = ''
    echo "=== Configuratarr E2E DevShell (autobrr-v1) ==="
    ${common}

    _AB_DATA=$(mktemp -d -t configuratarr-autobrr-XXXXXX)
    cat > "$_AB_DATA/config.toml" <<'EOF'
    host = "127.0.0.1"
    port = 7474
    sessionSecret = "configuratarre2econfiguratarre2e01"
    checkForUpdates = false
    logLevel = "ERROR"
    EOF

    if ! e2e_reclaim_port 7474 autobrr; then
      return 2>/dev/null || exit 1
    fi

    echo "  starting autobrr..."
    autobrr --config "$_AB_DATA" > "$_AB_DATA/autobrr.log" 2>&1 &
    _AB_PID=$!

    _ab_wait() {
      local i=0
      while [ $i -lt 60 ]; do
        curl -sf http://localhost:7474/api/healthz/liveness > /dev/null 2>&1 && return 0
        sleep 1; i=$((i + 1))
      done
      return 1
    }

    if _ab_wait; then
      _AB_CJ="$_AB_DATA/cookies"
      # Onboard the first user, log in for a session, then mint an API key.
      curl -sf -X POST http://localhost:7474/api/auth/onboard \
        -H 'Content-Type: application/json' \
        -d '{"username":"admin","password":"configuratarre2e"}' > /dev/null
      curl -sf -c "$_AB_CJ" -X POST http://localhost:7474/api/auth/login \
        -H 'Content-Type: application/json' \
        -d '{"username":"admin","password":"configuratarre2e"}' > /dev/null
      _AB_KEY=$(curl -sf -b "$_AB_CJ" -X POST http://localhost:7474/api/keys \
        -H 'Content-Type: application/json' \
        -d '{"name":"configuratarr","scopes":[]}' | jq -r '.key // empty' | head -n1)

      # Onboard/login/mint each fail quietly under `-sf … > /dev/null`; an empty
      # key then reaches the suite as a puzzling 401 rather than a setup error.
      if e2e_require "AUTOBRR_API_KEY" "$_AB_KEY"; then
        export AUTOBRR_URL="http://localhost:7474"
        export AUTOBRR_API_KEY="$_AB_KEY"
        echo "  autobrr ready — $AUTOBRR_URL"
        echo ""
        echo "  cargo nextest run -p autobrr-v1 --run-ignored all -j1"
      else
        echo "  autobrr onboarding failed — check $_AB_DATA/autobrr.log"
        kill "$_AB_PID" 2>/dev/null
      fi
    else
      echo "  autobrr failed to start — check $_AB_DATA/autobrr.log"
      kill "$_AB_PID" 2>/dev/null
    fi

    _ab_cleanup() {
      kill "$_AB_PID" 2>/dev/null
      wait "$_AB_PID" 2>/dev/null
      rm -rf "$_AB_DATA"
    }
    trap _ab_cleanup EXIT
  '';
}
