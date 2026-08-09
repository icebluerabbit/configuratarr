# Local e2e dev shell for prowlarr-v1.
# Starts Prowlarr with a known API key, exports PROWLARR_URL + PROWLARR_API_KEY,
# kills it and cleans up on exit.
{
  pkgs,
  e2eShell,
  common,
}:
pkgs.mkShell {
  inputsFrom = [ e2eShell ];
  shellHook = ''
    echo "=== Configuratarr E2E DevShell (prowlarr-v1) ==="
    ${common}

    _PROWLARR_DATA=$(mktemp -d -t configuratarr-prowlarr-XXXXXX)
    _PROWLARR_API_KEY="configuratarre2etestkey000000000"
    mkdir -p "$_PROWLARR_DATA"

    cat > "$_PROWLARR_DATA/config.xml" <<EOF
    <Config>
      <Port>9696</Port>
      <BindAddress>*</BindAddress>
      <ApiKey>$_PROWLARR_API_KEY</ApiKey>
      <AuthenticationMethod>None</AuthenticationMethod>
      <UpdateMechanism>External</UpdateMechanism>
      <AnalyticsEnabled>False</AnalyticsEnabled>
    </Config>
    EOF

    if ! e2e_reclaim_port 9696 Prowlarr; then
      return 2>/dev/null || exit 1
    fi

    echo "  starting Prowlarr..."
    HOME="$_PROWLARR_DATA" Prowlarr -nobrowser -data="$_PROWLARR_DATA" > "$_PROWLARR_DATA/prowlarr.log" 2>&1 &
    _PROWLARR_PID=$!

    _prowlarr_wait_ready() {
      local i=0
      while [ $i -lt 30 ]; do
        if curl -sf http://localhost:9696/api/v1/system/status \
             -H "X-Api-Key: $_PROWLARR_API_KEY" > /dev/null 2>&1; then
          return 0
        fi
        sleep 1
        i=$((i + 1))
      done
      return 1
    }

    if _prowlarr_wait_ready; then
      export PROWLARR_URL="http://localhost:9696"
      export PROWLARR_API_KEY="$_PROWLARR_API_KEY"
      echo "  Prowlarr ready — $PROWLARR_URL"
      echo ""
      echo "  cargo nextest run -p prowlarr-v1 --run-ignored all -j1"
    else
      echo "  Prowlarr failed to start — check $_PROWLARR_DATA/prowlarr.log"
      kill "$_PROWLARR_PID" 2>/dev/null
    fi

    _prowlarr_cleanup() {
      kill "$_PROWLARR_PID" 2>/dev/null
      wait "$_PROWLARR_PID" 2>/dev/null
      rm -rf "$_PROWLARR_DATA"
    }
    trap _prowlarr_cleanup EXIT
  '';
}
