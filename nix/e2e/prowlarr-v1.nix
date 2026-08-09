# nixosTest for prowlarr-v1. Called with the pre-built e2e test binary.
{ pkgs }:
e2eBin:
let
  # Prowlarr fetches its Cardigann indexer definitions from
  # indexers.prowlarr.com at runtime; the hermetic test VM has no network, so a
  # Cardigann-indexer create would 500 ("Name or service not known"). We seed the
  # on-disk Definitions cache (prowlarr's documented offline fallback) with the
  # real upstream set, pinned. v11 = prowlarr 2.4's schema version.
  indexerDefs = pkgs.fetchFromGitHub {
    owner = "Prowlarr";
    repo = "Indexers";
    rev = "59cafcccda7a2b3da27d3ec57026a62a658ea88c";
    hash = "sha256-p501MS2uwSZEy/MZG18OvlMToJUyUmr9gZXZjRv0CjQ=";
  };
in
pkgs.testers.nixosTest {
  name = "prowlarr-v1-e2e";
  nodes.machine = {
    services.prowlarr.enable = true;
    environment.systemPackages = [ e2eBin ];

    # Seed the definitions before prowlarr starts, as the service user, so its
    # startup "fallback to reading from disk" loads them into memory (the remote
    # download fails offline). Without this the on-demand fetch at indexer-create
    # time hits the network and 500s.
    systemd.services.prowlarr.preStart = ''
      mkdir -p /var/lib/prowlarr/Definitions
      cp -f ${indexerDefs}/definitions/v11/*.yml /var/lib/prowlarr/Definitions/
    '';
  };
  testScript = ''
    machine.wait_for_unit("prowlarr.service")
    machine.wait_for_open_port(9696, timeout=60)
    api_key = machine.wait_until_succeeds(
      "grep -oP '(?<=<ApiKey>)[^<]+' /var/lib/prowlarr/config.xml",
      timeout=30,
    ).strip()
    machine.succeed(
      f"PROWLARR_URL=http://localhost:9696 PROWLARR_API_KEY={api_key} "
      # --test-threads=1: e2e tests share one live instance (global tag list,
      # prune deletes all); parallel runs race. Run serially.
      f"prowlarr-v1-e2e --include-ignored --test-threads=1 2>&1"
    )
  '';
}
