# nixosTest for bindery-v1. Called with the pre-built e2e test binary.
#
# Bindery is not in nixpkgs, so both the package (`nix/pkgs/bindery.nix`) and a
# minimal module (`nix/modules/bindery.nix`) live in this repo.
#
# Unlike every other service here, no onboarding dance is needed: Bindery reads
# its instance API key straight from `BINDERY_API_KEY`, so the test fixes a
# known key and hands the same one to the e2e binary.
{ pkgs }:
e2eBin:
pkgs.testers.nixosTest {
  name = "bindery-v1-e2e";
  nodes.machine = {
    imports = [ ../modules/bindery.nix ];
    services.bindery = {
      enable = true;
      port = 8787;
      apiKey = "configuratarre2econfiguratarre2e";
    };
    environment.systemPackages = [
      e2eBin
      pkgs.curl
    ];
  };
  testScript = ''
    machine.wait_for_unit("bindery.service")
    machine.wait_for_open_port(8787, timeout=120)

    # /api/v1/health is unauthenticated and answers once the router is up.
    machine.wait_until_succeeds("curl -sf http://localhost:8787/api/v1/health", timeout=120)

    api_key = "configuratarre2econfiguratarre2e"

    # /api/v1/system/status *is* authenticated, so this also proves the key.
    machine.wait_until_succeeds(
      f"curl -sf http://localhost:8787/api/v1/system/status -H 'X-Api-Key: {api_key}'",
      timeout=60,
    )

    machine.succeed(
      f"BINDERY_URL=http://localhost:8787 BINDERY_API_KEY={api_key} "
      # --test-threads=1: e2e tests share one live instance (prune deletes
      # everything); parallel runs race.
      f"bindery-v1-e2e --include-ignored --test-threads=1 2>&1"
    )
  '';
}
