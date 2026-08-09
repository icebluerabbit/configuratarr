# nixosTest for radarr-v3. Called with the pre-built e2e test binary.
{ pkgs }:
e2eBin:
pkgs.testers.nixosTest {
  name = "radarr-v3-e2e";
  nodes.machine = {
    services.radarr.enable = true;
    environment.systemPackages = [ e2eBin ];
  };
  testScript = ''
    machine.wait_for_unit("radarr.service")
    machine.wait_for_open_port(7878, timeout=60)
    api_key = machine.wait_until_succeeds(
      "grep -oP '(?<=<ApiKey>)[^<]+' /var/lib/radarr/.config/Radarr/config.xml",
      timeout=30,
    ).strip()
    machine.succeed(
      f"RADARR_URL=http://localhost:7878 RADARR_API_KEY={api_key} "
      # --test-threads=1: e2e tests share one live instance (global tag list,
      # prune deletes all); parallel runs race. Run serially.
      f"radarr-v3-e2e --include-ignored --test-threads=1 2>&1"
    )
  '';
}
