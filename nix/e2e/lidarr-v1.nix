# nixosTest for lidarr-v1. Called with the pre-built e2e test binary.
{ pkgs }:
e2eBin:
pkgs.testers.nixosTest {
  name = "lidarr-v1-e2e";
  nodes.machine = {
    services.lidarr.enable = true;
    environment.systemPackages = [ e2eBin ];
  };
  testScript = ''
    machine.wait_for_unit("lidarr.service")
    machine.wait_for_open_port(8686, timeout=60)
    api_key = machine.wait_until_succeeds(
      "grep -oP '(?<=<ApiKey>)[^<]+' /var/lib/lidarr/.config/Lidarr/config.xml",
      timeout=30,
    ).strip()
    machine.succeed(
      f"LIDARR_URL=http://localhost:8686 LIDARR_API_KEY={api_key} "
      # --test-threads=1: e2e tests share one live instance (global tag list,
      # prune deletes all); parallel runs race. Run serially.
      f"lidarr-v1-e2e --include-ignored --test-threads=1 2>&1"
    )
  '';
}
