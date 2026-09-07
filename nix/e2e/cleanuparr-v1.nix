# nixosTest for cleanuparr-v1. Called with the pre-built e2e test binary.
#
# Cleanuparr is not in nixpkgs, so both the package and the `services.cleanuparr`
# module come from the `cleanuparr-flake` input.
#
# Like autobrr, Cleanuparr has no config-file API key: the key is minted with the
# admin account during first-run setup and is only readable through the API. The
# setup-guard middleware also makes ordering load-bearing — until
# `/api/auth/setup/complete` succeeds every non-auth `/api/*` path answers 403
# `{"error":"Setup required"}` — so we create the account, complete setup, log in
# for a JWT, then read the key and hand it to the e2e binary.
{
  pkgs,
  cleanuparr,
  cleanuparrModule,
}:
e2eBin:
pkgs.testers.nixosTest {
  name = "cleanuparr-v1-e2e";
  nodes.machine = {
    imports = [ cleanuparrModule ];

    services.cleanuparr = {
      enable = true;
      package = cleanuparr;
      port = 11011;
      bindAddress = "127.0.0.1";
    };

    environment.systemPackages = [
      e2eBin
      pkgs.curl
      pkgs.jq
    ];
  };
  testScript = ''
    from datetime import timedelta

    machine.wait_for_unit("cleanuparr.service")
    machine.wait_for_open_port(11011, timeout=timedelta(seconds=180))
    # /health is the unauthenticated liveness probe: bare text/plain, no JSON.
    machine.wait_until_succeeds(
      "curl -sf http://localhost:11011/health",
      timeout=timedelta(seconds=120),
    )

    # First-run setup, then log in and read the generated API key.
    machine.succeed(
      "curl -sf -X POST http://localhost:11011/api/auth/setup/account "
      "-H 'Content-Type: application/json' "
      "-d '{\"username\":\"admin\",\"password\":\"configuratarre2e\"}'"
    )
    machine.succeed("curl -sf -X POST http://localhost:11011/api/auth/setup/complete")
    jwt = machine.succeed(
      "curl -sf -X POST http://localhost:11011/api/auth/login "
      "-H 'Content-Type: application/json' "
      "-d '{\"username\":\"admin\",\"password\":\"configuratarre2e\"}' "
      "| jq -r '.tokens.accessToken' | head -n1"
    ).strip()
    api_key = machine.succeed(
      f"curl -sf http://localhost:11011/api/account/api-key "
      f"-H 'Authorization: Bearer {jwt}' | jq -r '.apiKey' | head -n1"
    ).strip()

    machine.succeed(
      f"CLEANUPARR_URL=http://localhost:11011 CLEANUPARR_API_KEY={api_key} "
      # --test-threads=1: e2e tests share one live instance; run serially.
      f"cleanuparr-v1-e2e --include-ignored --test-threads=1 2>&1"
    )
  '';
}
