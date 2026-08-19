# Bindery — automated book (ebook/audiobook) library manager, the modern
# replacement for Readarr. Not in nixpkgs, so the e2e VM test needs it built
# here.
#
# Two stages, mirroring upstream's Dockerfile: the React frontend in `web/` is
# built with npm, then the Go binary embeds `web/dist` at
# `internal/webui/dist` via `go:embed` and ships as a single executable.
{
  lib,
  buildGoModule,
  buildNpmPackage,
  fetchFromGitHub,
  nix-update-script,
}:
let
  version = "1.31.0";

  src = fetchFromGitHub {
    owner = "vavallee";
    repo = "bindery";
    tag = "v${version}";
    hash = "sha256-/ihcANTjhswzlDV2/IScLB69NzGtIQ+MsGSds+yYzSY=";
  };

  frontend = buildNpmPackage {
    pname = "bindery-web";
    inherit version src;
    sourceRoot = "${src.name}/web";
    npmDepsHash = "sha256-mFoaIS2QOTRDmEnJU7/h/S2vmKbmVoQG4KrnqyrZ3T8=";
    installPhase = ''
      runHook preInstall
      cp -r dist $out
      runHook postInstall
    '';
  };
in
buildGoModule {
  pname = "bindery";
  inherit version src;

  vendorHash = "sha256-qD/eVj4nbgcuVH8Nx2pvjm7T7mdXXA1fko9rVjVVtI8=";

  subPackages = [ "cmd/bindery" ];

  # `//go:embed all:dist` needs the built frontend in the tree before the Go
  # build runs; upstream's Dockerfile copies it to the same path.
  #
  # `internal/webui/dist` already exists in the source tree (it holds a
  # `.gitkeep` so the embed compiles), so this must copy the frontend's
  # *contents* into it — `cp -r ${frontend} internal/webui/dist` would nest them
  # one level deeper, and the binary then starts and immediately dies with
  # `failed to read embedded index.html`.
  preBuild = ''
    cp -r ${frontend}/. internal/webui/dist/
  '';

  ldflags = [
    "-s"
    "-w"
    "-X main.version=${version}"
  ];

  # The test suite spins up sqlite-backed fixtures and reaches for a writable
  # HOME; the e2e VM exercises the real binary, so skip them here.
  doCheck = false;

  passthru.updateScript = nix-update-script { };

  meta = {
    description = "Automated book download manager for Usenet, a modern replacement for Readarr";
    homepage = "https://github.com/vavallee/bindery";
    license = lib.licenses.mit;
    mainProgram = "bindery";
  };
}
