# Local e2e dev shell for sonarr-v3.
# Starts Sonarr with a known API key, exports SONARR_URL + SONARR_API_KEY,
# kills it and cleans up on exit.
{
  pkgs,
  e2eShell,
  common,
}:
pkgs.mkShell {
  inputsFrom = [ e2eShell ];
  shellHook = ''
    echo "=== Configuratarr E2E DevShell (sonarr-v3) ==="
    ${common}

    _SONARR_DATA=$(mktemp -d -t configuratarr-sonarr-XXXXXX)
    _SONARR_API_KEY="configuratarre2etestkey000000000"
    mkdir -p "$_SONARR_DATA/.config/NzbDrone"

    cat > "$_SONARR_DATA/.config/NzbDrone/config.xml" <<EOF
    <Config>
      <Port>8989</Port>
      <BindAddress>*</BindAddress>
      <ApiKey>$_SONARR_API_KEY</ApiKey>
      <AuthenticationMethod>None</AuthenticationMethod>
      <UpdateMechanism>External</UpdateMechanism>
      <AnalyticsEnabled>False</AnalyticsEnabled>
    </Config>
    EOF

    if ! e2e_reclaim_port 8989 Sonarr; then
      return 2>/dev/null || exit 1
    fi

    echo "  starting Sonarr..."
    HOME="$_SONARR_DATA" Sonarr -nobrowser -data="$_SONARR_DATA/.config/NzbDrone" > "$_SONARR_DATA/sonarr.log" 2>&1 &
    _SONARR_PID=$!

    _sonarr_wait_ready() {
      local i=0
      while [ $i -lt 30 ]; do
        if curl -sf http://localhost:8989/api/v3/system/status \
             -H "X-Api-Key: $_SONARR_API_KEY" > /dev/null 2>&1; then
          return 0
        fi
        sleep 1
        i=$((i + 1))
      done
      return 1
    }

    if _sonarr_wait_ready; then
      export SONARR_URL="http://localhost:8989"
      export SONARR_API_KEY="$_SONARR_API_KEY"
      echo "  Sonarr ready — $SONARR_URL"
      echo ""
      echo "  cargo nextest run -p sonarr-v3 --run-ignored all -j1"
    else
      echo "  Sonarr failed to start — check $_SONARR_DATA/sonarr.log"
      kill "$_SONARR_PID" 2>/dev/null
    fi

    _sonarr_cleanup() {
      kill "$_SONARR_PID" 2>/dev/null
      wait "$_SONARR_PID" 2>/dev/null
      rm -rf "$_SONARR_DATA"
    }
    trap _sonarr_cleanup EXIT
  '';
}
