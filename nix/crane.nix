# Crane build foundation shared across all crate derivations.
# Returns: { rustToolchain, craneLib, commonArgs, cargoArtifacts }
{
  pkgs,
  fenix,
  crane,
  root,
}:
let
  rustToolchain = fenix.packages.${pkgs.system}.stable.withComponents [
    "rustc"
    "cargo"
    "clippy"
    "rustfmt"
    "rust-src"
  ];

  craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

  # Include testdata (.json, .yaml) alongside Rust sources.
  src = pkgs.lib.cleanSourceWith {
    src = root;
    filter =
      path: type:
      (craneLib.filterCargoSources path type)
      || pkgs.lib.hasSuffix ".json" path
      || pkgs.lib.hasSuffix ".yaml" path;
  };

  commonArgs = {
    inherit src;
    strictDeps = true;
    nativeBuildInputs = [ pkgs.pkg-config ];
    buildInputs = [ pkgs.openssl ];
  };

  # Deps compiled once, reused by every downstream crane derivation.
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
{
  inherit
    rustToolchain
    craneLib
    commonArgs
    cargoArtifacts
    ;
}
