# nixosTest for audiobookshelf-v1. Called with the pre-built e2e test binary.
#
# Audiobookshelf has no API key: it issues a JWT access token at login, and the
# service authenticates with `Authorization: Bearer <token>`. A fresh install has
# no users, so we create the root user through the unauthenticated `/init` route,
# log in, and hand the access token to the e2e binary via AUDIOBOOKSHELF_TOKEN.
{ pkgs }:
e2eBin:
pkgs.testers.nixosTest {
  name = "audiobookshelf-v1-e2e";
  nodes.machine = {
    services.audiobookshelf = {
      enable = true;
      host = "127.0.0.1";
      port = 8000;
    };
    environment.systemPackages = [
      e2eBin
      pkgs.curl
      pkgs.jq
    ];
  };
  testScript = ''
    from datetime import timedelta

    machine.wait_for_unit("audiobookshelf.service")
    machine.wait_for_open_port(8000, timeout=timedelta(seconds=120))

    # /status is unauthenticated and reports whether a root user exists yet.
    machine.wait_until_succeeds(
      "curl -sf http://localhost:8000/status",
      timeout=timedelta(seconds=120),
    )

    # Create the root user (only possible while the server is uninitialised).
    machine.succeed(
      "curl -sf -X POST http://localhost:8000/init "
      "-H 'Content-Type: application/json' "
      '-d \'{"newRoot":{"username":"root","password":"configuratarre2e"}}\' '
    )

    # Log in for an access token — the credential the service actually uses.
    token = machine.wait_until_succeeds(
      "curl -sf -X POST http://localhost:8000/login "
      "-H 'Content-Type: application/json' "
      '-d \'{"username":"root","password":"configuratarre2e"}\' '
      "| jq -r '.user.accessToken'",
      timeout=timedelta(seconds=60),
    ).strip()

    # The token authenticates once this answers.
    machine.wait_until_succeeds(
      f"curl -sf http://localhost:8000/api/libraries -H 'Authorization: Bearer {token}'",
      timeout=timedelta(seconds=60),
    )

    machine.succeed(
      f"AUDIOBOOKSHELF_URL=http://localhost:8000 AUDIOBOOKSHELF_TOKEN={token} "
      # --test-threads=1: e2e tests share one live instance; parallel runs race.
      f"audiobookshelf-v1-e2e --include-ignored --test-threads=1 2>&1"
    )
  '';
}
