# Local e2e dev shell for lidarr-v1.
# Starts Lidarr with a known API key, exports LIDARR_URL + LIDARR_API_KEY,
# kills it and cleans up on exit.
{
  pkgs,
  e2eShell,
  common,
}:
pkgs.mkShell {
  inputsFrom = [ e2eShell ];
  shellHook = ''
    echo "=== Configuratarr E2E DevShell (lidarr-v1) ==="
    ${common}

    _LIDARR_DATA=$(mktemp -d -t configuratarr-lidarr-XXXXXX)
    _LIDARR_API_KEY="configuratarre2etestkey000000000"
    mkdir -p "$_LIDARR_DATA/.config/Lidarr"

    cat > "$_LIDARR_DATA/.config/Lidarr/config.xml" <<EOF
    <Config>
      <Port>8686</Port>
      <BindAddress>*</BindAddress>
      <ApiKey>$_LIDARR_API_KEY</ApiKey>
      <AuthenticationMethod>None</AuthenticationMethod>
      <UpdateMechanism>External</UpdateMechanism>
      <AnalyticsEnabled>False</AnalyticsEnabled>
    </Config>
    EOF

    if ! e2e_reclaim_port 8686 Lidarr; then
      return 2>/dev/null || exit 1
    fi

    echo "  starting Lidarr..."
    HOME="$_LIDARR_DATA" Lidarr -nobrowser -data="$_LIDARR_DATA/.config/Lidarr" > "$_LIDARR_DATA/lidarr.log" 2>&1 &
    _LIDARR_PID=$!

    _lidarr_wait_ready() {
      local i=0
      while [ $i -lt 30 ]; do
        if curl -sf http://localhost:8686/api/v1/system/status \
             -H "X-Api-Key: $_LIDARR_API_KEY" > /dev/null 2>&1; then
          return 0
        fi
        sleep 1
        i=$((i + 1))
      done
      return 1
    }

    if _lidarr_wait_ready; then
      export LIDARR_URL="http://localhost:8686"
      export LIDARR_API_KEY="$_LIDARR_API_KEY"
      echo "  Lidarr ready — $LIDARR_URL"
      echo ""
      echo "  cargo nextest run -p lidarr-v1 --run-ignored all -j1"
    else
      echo "  Lidarr failed to start — check $_LIDARR_DATA/lidarr.log"
      kill "$_LIDARR_PID" 2>/dev/null
    fi

    _lidarr_cleanup() {
      kill "$_LIDARR_PID" 2>/dev/null
      wait "$_LIDARR_PID" 2>/dev/null
      rm -rf "$_LIDARR_DATA"
    }
    trap _lidarr_cleanup EXIT
  '';
}
