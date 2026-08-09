# nixosTest for bazarr-v1. Called with the pre-built e2e test binary.
#
# Bazarr generates a random API key on first run and stores it in its config
# file (`<dataDir>/config/config.yaml`, or `config.ini` on older builds). The
# settings API needs that key, so we can't ask the API for it — we read it out
# of the config file once bazarr has written it, then hand it to the e2e binary.
{ pkgs }:
e2eBin:
pkgs.testers.nixosTest {
  name = "bazarr-v1-e2e";
  nodes.machine = {
    services.bazarr.enable = true;
    environment.systemPackages = [
      e2eBin
      pkgs.curl
      pkgs.gnugrep
    ];
  };
  testScript = ''
    machine.wait_for_unit("bazarr.service")
    machine.wait_for_open_port(6767, timeout=180)

    # Bazarr writes its config (with the generated apikey) shortly after start.
    machine.wait_until_succeeds(
      "grep -qsE 'apikey' /var/lib/bazarr/config/config.yaml /var/lib/bazarr/config/config.ini",
      timeout=120,
    )
    # bazarr writes only one of config.yaml / config.ini; grep exits 2 on the
    # absent one, which fails under the test shell's `pipefail` even though the
    # key was matched. Swallow that with `|| true` (an empty result still fails
    # loudly at the X-API-KEY curl below); `-m1` stops at the first match.
    api_key = machine.succeed(
      "{ grep -hoE -m1 'apikey[:=] *[A-Za-z0-9]+' "
      "/var/lib/bazarr/config/config.yaml /var/lib/bazarr/config/config.ini 2>/dev/null "
      "|| true; } | grep -oE '[A-Za-z0-9]+$'"
    ).strip()

    # The API answers once bazarr is fully up (authenticated by the key).
    machine.wait_until_succeeds(
      f"curl -sf http://localhost:6767/api/system/status -H 'X-API-KEY: {api_key}'",
      timeout=120,
    )

    machine.succeed(
      f"BAZARR_URL=http://localhost:6767 BAZARR_API_KEY={api_key} "
      # --test-threads=1: e2e tests share one live instance; run serially.
      f"bazarr-v1-e2e --include-ignored --test-threads=1 2>&1"
    )
  '';
}
