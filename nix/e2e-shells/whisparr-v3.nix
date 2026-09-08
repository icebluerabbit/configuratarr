# Local e2e dev shell for whisparr-v3.
#
# Whisparr eros (V3) is not in nixpkgs — `pkgs.whisparr` is the V2 Sonarr fork —
# so this builds it from `nix/pkgs/whisparr-eros.nix`, the same package the
# e2e VM test boots.
#
# Same shape as the radarr shell: seed a `config.xml` with a known API key,
# start the app in a temp data dir, export WHISPARR_URL + WHISPARR_API_KEY, kill
# it and clean up on exit.
{
  pkgs,
  e2eShell,
  common,
}:
let
  whisparr = pkgs.callPackage ../pkgs/whisparr-eros.nix { };
in
pkgs.mkShell {
  inputsFrom = [ e2eShell ];
  packages = [ whisparr ];
  shellHook = ''
    echo "=== Configuratarr E2E DevShell (whisparr-v3) ==="
    ${common}

    _WHISPARR_DATA=$(mktemp -d -t configuratarr-whisparr-XXXXXX)
    _WHISPARR_API_KEY="configuratarre2etestkey000000000"

    cat > "$_WHISPARR_DATA/config.xml" <<EOF
    <Config>
      <Port>6969</Port>
      <BindAddress>*</BindAddress>
      <ApiKey>$_WHISPARR_API_KEY</ApiKey>
      <AuthenticationMethod>None</AuthenticationMethod>
      <UpdateMechanism>External</UpdateMechanism>
      <AnalyticsEnabled>False</AnalyticsEnabled>
    </Config>
    EOF

    if ! e2e_reclaim_port 6969 Whisparr; then
      return 2>/dev/null || exit 1
    fi

    echo "  starting Whisparr (eros)..."
    ${pkgs.lib.getExe whisparr} -nobrowser -data="$_WHISPARR_DATA" \
      > "$_WHISPARR_DATA/whisparr.log" 2>&1 &
    _WHISPARR_PID=$!

    _whisparr_wait_ready() {
      local i=0
      # 241 FluentMigrator migrations run against a fresh database before the
      # listener opens, so this is slower to come up than the *arr binaries.
      while [ $i -lt 90 ]; do
        if curl -sf http://localhost:6969/api/v3/system/status \
             -H "X-Api-Key: $_WHISPARR_API_KEY" > /dev/null 2>&1; then
          return 0
        fi
        sleep 1
        i=$((i + 1))
      done
      return 1
    }

    if _whisparr_wait_ready; then
      export WHISPARR_URL="http://localhost:6969"
      export WHISPARR_API_KEY="$_WHISPARR_API_KEY"
      echo "  Whisparr ready — $WHISPARR_URL"
      echo ""
      echo "  cargo nextest run -p whisparr-v3 --run-ignored all -j1"
    else
      echo "  Whisparr failed to start — check $_WHISPARR_DATA/whisparr.log"
      kill "$_WHISPARR_PID" 2>/dev/null
    fi

    _whisparr_cleanup() {
      kill "$_WHISPARR_PID" 2>/dev/null
      wait "$_WHISPARR_PID" 2>/dev/null
      rm -rf "$_WHISPARR_DATA"
    }
    trap _whisparr_cleanup EXIT
  '';
}
