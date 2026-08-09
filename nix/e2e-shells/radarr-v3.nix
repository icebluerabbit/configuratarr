# Local e2e dev shell for radarr-v3.
# Starts Radarr with a known API key, exports RADARR_URL + RADARR_API_KEY,
# kills it and cleans up on exit.
{
  pkgs,
  e2eShell,
  common,
}:
pkgs.mkShell {
  inputsFrom = [ e2eShell ];
  shellHook = ''
    echo "=== Configuratarr E2E DevShell (radarr-v3) ==="
    ${common}

    _RADARR_DATA=$(mktemp -d -t configuratarr-radarr-XXXXXX)
    _RADARR_API_KEY="configuratarre2etestkey000000000"

    cat > "$_RADARR_DATA/config.xml" <<EOF
    <Config>
      <Port>7878</Port>
      <BindAddress>*</BindAddress>
      <ApiKey>$_RADARR_API_KEY</ApiKey>
      <AuthenticationMethod>None</AuthenticationMethod>
      <UpdateMechanism>External</UpdateMechanism>
      <AnalyticsEnabled>False</AnalyticsEnabled>
    </Config>
    EOF

    if ! e2e_reclaim_port 7878 Radarr; then
      return 2>/dev/null || exit 1
    fi

    echo "  starting Radarr..."
    Radarr -nobrowser -data="$_RADARR_DATA" > "$_RADARR_DATA/radarr.log" 2>&1 &
    _RADARR_PID=$!

    _radarr_wait_ready() {
      local i=0
      while [ $i -lt 30 ]; do
        if curl -sf http://localhost:7878/api/v3/system/status \
             -H "X-Api-Key: $_RADARR_API_KEY" > /dev/null 2>&1; then
          return 0
        fi
        sleep 1
        i=$((i + 1))
      done
      return 1
    }

    if _radarr_wait_ready; then
      export RADARR_URL="http://localhost:7878"
      export RADARR_API_KEY="$_RADARR_API_KEY"
      echo "  Radarr ready — $RADARR_URL"
      echo ""
      echo "  cargo nextest run -p radarr-v3 --run-ignored all -j1"
    else
      echo "  Radarr failed to start — check $_RADARR_DATA/radarr.log"
      kill "$_RADARR_PID" 2>/dev/null
    fi

    _radarr_cleanup() {
      kill "$_RADARR_PID" 2>/dev/null
      wait "$_RADARR_PID" 2>/dev/null
      rm -rf "$_RADARR_DATA"
    }
    trap _radarr_cleanup EXIT
  '';
}
