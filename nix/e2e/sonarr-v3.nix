# nixosTest for sonarr-v3. Called with the pre-built e2e test binary.
{ pkgs }:
e2eBin:
pkgs.testers.nixosTest {
  name = "sonarr-v3-e2e";
  nodes.machine = {
    services.sonarr.enable = true;
    environment.systemPackages = [ e2eBin ];
  };
  testScript = ''
    machine.wait_for_unit("sonarr.service")
    machine.wait_for_open_port(8989, timeout=60)
    api_key = machine.wait_until_succeeds(
      "grep -oP '(?<=<ApiKey>)[^<]+' /var/lib/sonarr/.config/NzbDrone/config.xml",
      timeout=30,
    ).strip()
    machine.succeed(
      f"SONARR_URL=http://localhost:8989 SONARR_API_KEY={api_key} "
      # --test-threads=1: e2e tests share one live instance (global tag list,
      # prune deletes all); parallel runs race. Run serially.
      f"sonarr-v3-e2e --include-ignored --test-threads=1 2>&1"
    )
  '';
}
