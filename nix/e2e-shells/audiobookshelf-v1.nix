# Local e2e dev shell for audiobookshelf-v1.
#
# Audiobookshelf has no API key — it issues a JWT access token at login. So this
# hook starts ABS in a temp data dir, creates the root user through the
# unauthenticated `/init` route, logs in, exports AUDIOBOOKSHELF_URL +
# AUDIOBOOKSHELF_TOKEN, and cleans up on exit.
{
  pkgs,
  e2eShell,
  common,
}:
pkgs.mkShell {
  inputsFrom = [ e2eShell ];
  packages = [
    pkgs.audiobookshelf
    pkgs.curl
    pkgs.jq
  ];
  shellHook = ''
    echo "=== Configuratarr E2E DevShell (audiobookshelf-v1) ==="
    ${common}

    _ABS_DATA=$(mktemp -d -t configuratarr-audiobookshelf-XXXXXX)
    _ABS_URL="http://localhost:8000"
    _ABS_USER="root"
    _ABS_PASS="configuratarre2e"

    # ABS takes its paths from the environment, so the temp dir never reaches the
    # command line — pass it explicitly for the leaked-instance check.
    if ! e2e_reclaim_port 8000 Audiobookshelf "*$_ABS_DATA*"; then
      return 2>/dev/null || exit 1
    fi

    echo "  starting Audiobookshelf..."
    mkdir -p "$_ABS_DATA/config" "$_ABS_DATA/metadata"
    # Flags, NOT environment variables. nixpkgs wraps Audiobookshelf in a shell
    # script that sets CONFIG_PATH/METADATA_PATH/PORT/HOST itself, overwriting
    # anything the caller exported, and its defaults are `$(pwd)/config` and
    # `$(pwd)/metadata`. Passing env vars therefore does nothing except let it
    # write a whole SQLite database and log tree into the current directory —
    # i.e. into this repo, when the shell is entered from the repo root.
    audiobookshelf \
      --host 127.0.0.1 \
      --port 8000 \
      --config "$_ABS_DATA/config" \
      --metadata "$_ABS_DATA/metadata" \
      > "$_ABS_DATA/abs.log" 2>&1 &
    _ABS_PID=$!

    _abs_wait() {
      local i=0
      while [ $i -lt 90 ]; do
        curl -sf "$_ABS_URL/status" > /dev/null 2>&1 && return 0
        sleep 1; i=$((i + 1))
      done
      return 1
    }

    if _abs_wait; then
      # Only possible while the server is uninitialised; a rerun 500s, which is
      # fine — the user already exists and the login below still works.
      curl -sf -X POST "$_ABS_URL/init" \
        -H 'Content-Type: application/json' \
        -d "{\"newRoot\":{\"username\":\"$_ABS_USER\",\"password\":\"$_ABS_PASS\"}}" \
        > /dev/null 2>&1 || true

      _ABS_TOKEN=$(curl -sf -X POST "$_ABS_URL/login" \
        -H 'Content-Type: application/json' \
        -d "{\"username\":\"$_ABS_USER\",\"password\":\"$_ABS_PASS\"}" \
        | jq -r '.user.accessToken')
    fi

    if e2e_require "AUDIOBOOKSHELF_TOKEN" "$_ABS_TOKEN"; then
      export AUDIOBOOKSHELF_URL="$_ABS_URL"
      export AUDIOBOOKSHELF_TOKEN="$_ABS_TOKEN"
      echo "  Audiobookshelf ready — $AUDIOBOOKSHELF_URL"
      echo ""
      echo "  cargo nextest run -p audiobookshelf-v1 --run-ignored all -j1"
    else
      echo "  Audiobookshelf failed to start — check $_ABS_DATA/abs.log"
      kill "$_ABS_PID" 2>/dev/null
    fi

    _abs_cleanup() {
      kill "$_ABS_PID" 2>/dev/null
      wait "$_ABS_PID" 2>/dev/null
      rm -rf "$_ABS_DATA"
    }
    trap _abs_cleanup EXIT
  '';
}
