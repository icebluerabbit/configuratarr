# Local e2e dev shell for komga-v1.
#
# Komga starts with no users: the first account is created by claiming the
# server, and API keys are minted afterwards over HTTP Basic as that user. So
# this hook starts Komga in a temp data dir, claims it, mints a key, exports
# KOMGA_URL + KOMGA_API_KEY, and cleans up on exit.
{
  pkgs,
  e2eShell,
  common,
}:
pkgs.mkShell {
  inputsFrom = [ e2eShell ];
  packages = [
    pkgs.komga
    pkgs.curl
    pkgs.jq
  ];
  shellHook = ''
    echo "=== Configuratarr E2E DevShell (komga-v1) ==="
    ${common}

    _KO_DATA=$(mktemp -d -t configuratarr-komga-XXXXXX)
    _KO_URL="http://localhost:25600"
    _KO_USER="e2e@configuratarr.test"
    _KO_PASS="configuratarre2e"

    if ! e2e_reclaim_port 25600 Komga "*$_KO_DATA*"; then
      return 2>/dev/null || exit 1
    fi

    echo "  starting Komga..."
    # `KOMGA_CONFIGDIR` (Spring relaxed binding for `komga.config-dir`) is the
    # supported way to point Komga at a data directory. Do NOT reach for
    # `--spring.config.location` instead: that *replaces* the default config
    # locations rather than adding to them, so Komga's own bundled
    # `application.yml` never loads — and with it go `spring.flyway.mixed: true`
    # and the migration placeholders, leaving startup to die in Flyway with
    # "Detected both transactional and non-transactional statements within the
    # same migration".
    KOMGA_CONFIGDIR="$_KO_DATA" \
    KOMGA_DATABASE_FILE="$_KO_DATA/database.sqlite" \
    SERVER_PORT=25600 \
    LOGGING_FILE_NAME="$_KO_DATA/komga.log" \
      komga > "$_KO_DATA/komga-stdout.log" 2>&1 &
    _KO_PID=$!

    # The JVM takes a while; /api/v1/claim is unauthenticated and answers once
    # the app context is up.
    _ko_wait() {
      local i=0
      while [ $i -lt 180 ]; do
        curl -sf "$_KO_URL/api/v1/claim" > /dev/null 2>&1 && return 0
        sleep 1; i=$((i + 1))
      done
      return 1
    }

    if _ko_wait; then
      # Claim creates the first (admin) user; already-claimed is fine on a rerun.
      curl -sf -X POST "$_KO_URL/api/v1/claim" \
        -H "X-Komga-Email: $_KO_USER" \
        -H "X-Komga-Password: $_KO_PASS" > /dev/null 2>&1 || true

      _KO_KEY=$(curl -sf -u "$_KO_USER:$_KO_PASS" \
        -X POST "$_KO_URL/api/v2/users/me/api-keys" \
        -H 'Content-Type: application/json' \
        -d '{"comment":"configuratarr-e2e"}' | jq -r '.key')
    fi

    if e2e_require "KOMGA_API_KEY" "$_KO_KEY"; then
      export KOMGA_URL="$_KO_URL"
      export KOMGA_API_KEY="$_KO_KEY"
      echo "  Komga ready — $KOMGA_URL"
      echo ""
      echo "  cargo nextest run -p komga-v1 --run-ignored all -j1"
    else
      echo "  Komga failed to start — check $_KO_DATA/komga-stdout.log"
      kill "$_KO_PID" 2>/dev/null
    fi

    _ko_cleanup() {
      kill "$_KO_PID" 2>/dev/null
      wait "$_KO_PID" 2>/dev/null
      rm -rf "$_KO_DATA"
    }
    trap _ko_cleanup EXIT
  '';
}
