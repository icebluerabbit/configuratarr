# Returns checks for one API crate: unit tests (all platforms) + e2e nixosTest (Linux only).
#
# Usage:
#   mkServiceChecks "radarr-v3" (import ./e2e/radarr-v3.nix { inherit pkgs; })
#
# Adding a new service:
#   checks = mkServiceChecks "radarr-v3" radarrE2e
#     // mkServiceChecks "sonarr-v3" sonarrE2e;
{
  pkgs,
  craneLib,
  commonArgs,
  cargoArtifacts,
}:
crateName: mkNixosTest:
let
  unitTests = craneLib.cargoNextest (
    commonArgs
    // {
      inherit cargoArtifacts;
      cargoNextestExtraArgs = "-p ${crateName}";
    }
  );

  e2eBin = craneLib.mkCargoDerivation (
    commonArgs
    // {
      inherit cargoArtifacts;
      pname = "${crateName}-e2e";
      version = "0.1.0";
      buildPhaseCargoCommand = "cargo test --no-run -p ${crateName} --test e2e";
      installPhaseCommand = ''
        mkdir -p $out/bin
        find target/debug/deps -maxdepth 1 -executable -name 'e2e-*' \
          -exec cp {} $out/bin/${crateName}-e2e \;
      '';
      doInstallCargoArtifacts = false;
    }
  );
in
{
  "${crateName}-unit-tests" = unitTests;
}
// pkgs.lib.optionalAttrs pkgs.stdenv.isLinux {
  "${crateName}-e2e" = mkNixosTest e2eBin;
}
