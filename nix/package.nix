{
  lib,
  rustPlatform,
  pkg-config,
  openssl,
  docs ? null,
}:

rustPlatform.buildRustPackage {
  pname = "configuratarr";
  version = "0.1.0";
  src = ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];

  passthru = lib.optionalAttrs (docs != null) {
    inherit docs;
  };
}
