# nixosTest for autobrr-v1. Called with the pre-built e2e test binary.
#
# autobrr has no config-file API key (unlike bazarr): keys are minted through the
# API after onboarding the first user. So we onboard, log in for a session, then
# POST a key and hand it to the e2e binary via AUTOBRR_API_KEY.
{ pkgs }:
e2eBin:
pkgs.testers.nixosTest {
  name = "autobrr-v1-e2e";
  nodes.machine = {
    services.autobrr = {
      enable = true;
      secretFile = pkgs.writeText "autobrr-session-secret" "configuratarre2econfiguratarre2e01";
      settings = {
        host = "127.0.0.1";
        port = 7474;
        checkForUpdates = false;
        logLevel = "ERROR";
      };
    };
    environment.systemPackages = [
      e2eBin
      pkgs.curl
      pkgs.jq
    ];
  };
  testScript = ''
    machine.wait_for_unit("autobrr.service")
    machine.wait_for_open_port(7474, timeout=180)
    machine.wait_until_succeeds(
      "curl -sf http://localhost:7474/api/healthz/liveness",
      timeout=120,
    )

    # Onboard the first user, log in for a session cookie, mint an API key.
    machine.succeed(
      "curl -sf -X POST http://localhost:7474/api/auth/onboard "
      "-H 'Content-Type: application/json' "
      "-d '{\"username\":\"admin\",\"password\":\"configuratarre2e\"}'"
    )
    machine.succeed(
      "curl -sf -c /tmp/cj -X POST http://localhost:7474/api/auth/login "
      "-H 'Content-Type: application/json' "
      "-d '{\"username\":\"admin\",\"password\":\"configuratarre2e\"}'"
    )
    api_key = machine.succeed(
      "curl -sf -b /tmp/cj -X POST http://localhost:7474/api/keys "
      "-H 'Content-Type: application/json' "
      "-d '{\"name\":\"configuratarr\",\"scopes\":[]}' "
      "| jq -r '.key' | head -n1"
    ).strip()

    machine.succeed(
      f"AUTOBRR_URL=http://localhost:7474 AUTOBRR_API_KEY={api_key} "
      # --test-threads=1: e2e tests share one live instance; run serially.
      f"autobrr-v1-e2e --include-ignored --test-threads=1 2>&1"
    )
  '';
}
