# nixosTest for komga-v1. Called with the pre-built e2e test binary.
#
# Komga starts with no users at all: the first account is created by "claiming"
# the server (`POST /api/v1/claim`, credentials in headers). API keys are minted
# through the API afterwards, authenticated with HTTP Basic as that first user —
# so we claim, mint, and hand the key to the e2e binary via KOMGA_API_KEY.
{ pkgs }:
e2eBin:
pkgs.testers.nixosTest {
  name = "komga-v1-e2e";
  nodes.machine = {
    services.komga = {
      enable = true;
      settings.server.port = 25600;
    };
    # The nixpkgs module sets `PrivateTmp = true`, giving Komga its own `/tmp`
    # namespace. The library tests create a root directory under `/tmp`, and
    # Komga has to be able to see it — otherwise the library is created
    # `unavailable` and the paths don't line up.
    systemd.services.komga.serviceConfig.PrivateTmp = pkgs.lib.mkForce false;
    environment.systemPackages = [
      e2eBin
      pkgs.curl
      pkgs.jq
    ];
  };
  testScript = ''
    machine.wait_for_unit("komga.service")
    machine.wait_for_open_port(25600, timeout=300)

    # Komga answers /api/v1/claim (unauthenticated) once the app context is up;
    # the JVM start is slow, so poll rather than assuming the open port is ready.
    machine.wait_until_succeeds(
      "curl -sf http://localhost:25600/api/v1/claim", timeout=300
    )

    # Claim the server — creates the first (admin) user.
    machine.succeed(
      "curl -sf -X POST http://localhost:25600/api/v1/claim "
      "-H 'X-Komga-Email: e2e@configuratarr.test' "
      "-H 'X-Komga-Password: configuratarre2e'"
    )

    # Mint an API key as that user (Basic auth is the only credential we have).
    api_key = machine.succeed(
      "curl -sf -u e2e@configuratarr.test:configuratarre2e "
      "-X POST http://localhost:25600/api/v2/users/me/api-keys "
      "-H 'Content-Type: application/json' "
      "-d '{\"comment\":\"configuratarr-e2e\"}' "
      "| jq -r '.key'"
    ).strip()

    # The key authenticates (and the app is fully up) once this answers.
    machine.wait_until_succeeds(
      f"curl -sf http://localhost:25600/api/v2/users/me -H 'X-API-Key: {api_key}'",
      timeout=120,
    )

    machine.succeed(
      f"KOMGA_URL=http://localhost:25600 KOMGA_API_KEY={api_key} "
      # --test-threads=1: e2e tests share one live instance (prune deletes
      # everything); parallel runs race.
      f"komga-v1-e2e --include-ignored --test-threads=1 2>&1"
    )
  '';
}
