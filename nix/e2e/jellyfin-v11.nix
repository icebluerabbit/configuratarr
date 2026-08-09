# nixosTest for jellyfin-v11. Called with the pre-built e2e test binary.
#
# Jellyfin has no static-config API key (unlike the *arr apps). We bootstrap a
# fresh instance through the startup wizard, authenticate the admin to obtain an
# access token, then mint a real API key via `/Auth/Keys` and hand it to the
# e2e binary.
#
# NOTE: the exact `Authorization` header form Jellyfin accepts (raw key vs
# `MediaBrowser Token="…"`) is the open e2e risk for this service — see the
# crate's e2e.rs. This script uses the `MediaBrowser` scheme for the bootstrap
# curls; the engine sends the raw key, which this run validates.
{ pkgs }:
e2eBin:
pkgs.testers.nixosTest {
  name = "jellyfin-v11-e2e";
  nodes.machine = {
    services.jellyfin.enable = true;
    environment.systemPackages = [
      e2eBin
      pkgs.curl
      pkgs.jq
    ];
    # Jellyfin wants some headroom for first-run.
    virtualisation.memorySize = 2048;
    # Jellyfin 10.11 refuses to start unless its data dir has >= 2 GiB free
    # (StorageHelper.TestDataDirectorySize) — the default test disk is too small.
    virtualisation.diskSize = 4096;
    # The library e2e adds a media library; Jellyfin 400s a library whose path
    # doesn't exist on disk, so pre-create it (owned by the jellyfin user).
    systemd.tmpfiles.rules = [ "d /tmp/configuratarr-e2e-media 0755 jellyfin jellyfin -" ];
    # jellyfin.service runs with PrivateTmp=true, which would hide the /tmp dir
    # created above inside its private namespace — disable it for the test VM.
    systemd.services.jellyfin.serviceConfig.PrivateTmp = pkgs.lib.mkForce false;
  };
  testScript = ''
    import json

    machine.wait_for_unit("jellyfin.service")
    machine.wait_for_open_port(8096, timeout=180)
    machine.wait_until_succeeds("curl -sf http://localhost:8096/System/Info/Public", timeout=180)

    auth = 'Authorization: MediaBrowser Client="cfg-e2e", Device="cfg-e2e", DeviceId="cfg-e2e", Version="1"'

    # `/System/Info/Public` answers while Kestrel is up but Jellyfin may still be
    # running DB migrations, during which the startup-wizard controller 500s —
    # so poll the first startup call until it's actually ready.
    #
    # 300s, not 120s: on a slow CI runner Jellyfin spends ~65s in "Update Plugins"
    # failing to reach the plugin manifest (the test VM has no network) and then
    # ~85s probing ffmpeg codecs before this controller answers. 120s left no
    # margin and this timed out intermittently while passing locally and on
    # re-runs. `wait_until_succeeds` returns as soon as the call works, so a
    # larger budget costs nothing when the machine is fast.
    machine.wait_until_succeeds(f"curl -sf http://localhost:8096/Startup/Configuration -H '{auth}'", timeout=300)

    # Complete the startup wizard. The steps must run in order — POSTing the
    # admin user before priming the config/user GETs 404s.
    machine.succeed(
      "curl -sf -X POST http://localhost:8096/Startup/Configuration "
      "-H 'Content-Type: application/json' "
      f"-H '{auth}' "
      "-d '{\"UICulture\":\"en-US\",\"MetadataCountryCode\":\"US\",\"PreferredMetadataLanguage\":\"en\"}'"
    )
    machine.succeed(f"curl -sf http://localhost:8096/Startup/User -H '{auth}'")
    machine.succeed(
      "curl -sf -X POST http://localhost:8096/Startup/User "
      "-H 'Content-Type: application/json' "
      f"-H '{auth}' "
      "-d '{\"Name\":\"admin\",\"Password\":\"configuratarre2e\"}'"
    )
    machine.succeed(f"curl -sf -X POST http://localhost:8096/Startup/Complete -H '{auth}'")

    # Authenticate the admin → access token.
    out = machine.succeed(
      "curl -sf -X POST http://localhost:8096/Users/AuthenticateByName "
      "-H 'Content-Type: application/json' "
      f"-H '{auth}' "
      "-d '{\"Username\":\"admin\",\"Pw\":\"configuratarre2e\"}'"
    )
    token = json.loads(out)["AccessToken"]

    # Mint an API key, then read it back out of the key list.
    machine.succeed(
      "curl -sf -X POST 'http://localhost:8096/Auth/Keys?app=configuratarr' "
      f"-H 'Authorization: MediaBrowser Token=\"{token}\"'"
    )
    keys = machine.succeed(
      "curl -sf http://localhost:8096/Auth/Keys "
      f"-H 'Authorization: MediaBrowser Token=\"{token}\"'"
    )
    api_key = next(
      k["AccessToken"] for k in json.loads(keys)["Items"] if k["AppName"] == "configuratarr"
    )

    # Gate on the *authenticated* endpoint the engine actually uses (raw key in
    # `X-Emby-Token` against `/System/Info`, its declared health) settling before
    # handing off — `/System/Info/Public` answers while Jellyfin is still settling
    # after the startup wizard, so the first apply could catch a post-wizard flap.
    # The inner loop requires 3 consecutive OKs; `wait_until_succeeds` retries the
    # whole check until it holds, so a single lucky response doesn't count.
    machine.wait_until_succeeds(
      "for _ in 1 2 3; do "
      f"curl -sf http://localhost:8096/System/Info -H 'X-Emby-Token: {api_key}' >/dev/null "
      "|| exit 1; sleep 1; done",
      timeout=300,
    )

    machine.succeed(
      f"JELLYFIN_URL=http://localhost:8096 JELLYFIN_API_KEY={api_key} "
      # --test-threads=1: e2e tests share one live instance; run serially.
      f"jellyfin-v11-e2e --include-ignored --test-threads=1 2>&1"
    )
  '';
}
