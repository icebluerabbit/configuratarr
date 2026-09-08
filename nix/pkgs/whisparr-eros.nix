# Whisparr "eros" — the V3, Radarr-derived line (movies / performers / studios).
#
# WHY THIS EXISTS — do not replace it with `pkgs.whisparr`:
#
#   * `nixpkgs#whisparr` is 2.0.0.2151, which is Whisparr **V2**: a *Sonarr*
#     fork whose API is series/episodes. The `whisparr-v3` crate here targets
#     eros, whose API is movies/performers/studios. Booting V2 under the eros
#     e2e test fails every assertion in a confusing way.
#   * There is no `whisparr-v3` attribute in nixpkgs, and every GitHub release
#     on the repo is a `v2.2.0-develop.*` tag — V2 only.
#   * The Servarr update channel advertises eros 3.1.0.2116, but its artifact is
#     gone: the Azure DevOps build behind it has expired ("The requested build
#     5751 could not be found"). So the `fetchurl`-a-release-tarball approach
#     nixpkgs uses for the *arr family is not available here.
#   * Docker/OCI images are out of scope — every e2e test in `nix/e2e/` boots a
#     native NixOS module.
#
# That leaves building from source, pinned by commit for reproducibility.
#
# Two stages, mirroring upstream's `build.sh`:
#   1. `yarn run build --env production` — webpack emits the UI into
#      `_output/UI`.
#   2. `dotnet msbuild -restore src/Whisparr.sln -t:PublishAllRids` — .NET 8.
#      We publish the two projects that matter instead of the whole solution
#      (see `projectFile` below).
{
  lib,
  buildDotnetModule,
  dotnetCorePackages,
  fetchFromGitHub,
  fetchYarnDeps,
  nodejs,
  stdenvNoCC,
  yarn,
  yarnConfigHook,

  openssl,
  sqlite,
  zlib,
}:
let
  # eros' `package.json` says 3.1.0; the fourth component is stamped by CI from
  # the Azure build number, which we have no equivalent of. Pin the tree by
  # commit and carry the date, nixpkgs-style.
  version = "3.1.0-unstable-2026-08-28";

  # Must be a plain `System.Version`: it lands in `AssemblyVersion`, which
  # `Directory.Build.props` otherwise leaves as the placeholder `10.0.0.*`.
  # A wildcard there makes the build non-deterministic and reports a nonsense
  # version over `/api/v3/system/status`.
  assemblyVersion = "3.1.0.2116";

  src = fetchFromGitHub {
    owner = "Whisparr";
    repo = "Whisparr";
    rev = "cc3fb2abcf60f7c0048eb0294015d291b82bde08";
    hash = "sha256-4EWOWREF7PfWJDuTTvSYFTz7pWVFFflFIGiWZMvtILA=";
  };

  # `package.json` / `yarn.lock` sit at the repo root (sources under
  # `frontend/`), so the whole tree is the sourceRoot — yarnConfigHook looks for
  # the lockfile there.
  frontend = stdenvNoCC.mkDerivation {
    pname = "whisparr-eros-ui";
    inherit version src;

    nativeBuildInputs = [
      nodejs
      yarn
      yarnConfigHook
    ];

    yarnOfflineCache = fetchYarnDeps {
      yarnLock = "${src}/yarn.lock";
      hash = "sha256-WxAW/AEi8fvGykU4xBXgIxtsdcp41tYRK4Bp3MOpHuA=";
    };

    # The production webpack run holds the whole module graph plus terser in
    # memory; node's default heap is not enough.
    env.NODE_OPTIONS = "--max-old-space-size=4096";

    buildPhase = ''
      runHook preBuild
      yarn --offline run build --env production
      runHook postBuild
    '';

    # webpack's `distFolder` is `<repo>/_output/UI` — the same path the backend
    # expects to find next to its assemblies at runtime.
    installPhase = ''
      runHook preInstall
      cp -r _output/UI $out
      runHook postInstall
    '';
  };
in
buildDotnetModule {
  pname = "whisparr-eros";
  inherit version src;

  # `global.json` pins SDK 8.0.405; the 8.0 SDK in nixpkgs satisfies it once
  # roll-forward is allowed, which it is by default for patch versions.
  dotnet-sdk = dotnetCorePackages.sdk_8_0;
  dotnet-runtime = dotnetCorePackages.aspnetcore_8_0;

  # `Whisparr.Console` is the entry point. `Whisparr.Mono` is *not* referenced
  # by it — `AssemblyLoader` loads "Whisparr.Mono" by name at runtime on
  # non-Windows — so it has to be published alongside or the host dies during
  # container composition.
  projectFile = [
    "src/NzbDrone.Console/Whisparr.Console.csproj"
    "src/NzbDrone.Mono/Whisparr.Mono.csproj"
  ];
  nugetDeps = ./whisparr-eros-deps.json;

  # Several Servarr forks (Mono.Posix.NETStandard, System.Data.SQLite.Core,
  # FluentMigrator, FFMpegCore) only exist on Servarr's Azure DevOps feeds,
  # which `src/NuGet.config` declares. Those feeds are anonymously readable, so
  # restore works — but `nuget-to-json` (what `passthru.fetch-deps` runs) calls
  # `dotnet nuget list source` from the *source root*, where only the root
  # nuget.config is in scope. Without this copy it sees nuget.org alone and dies
  # with `couldn't find mono.posix.netstandard 5.20.1.34-servarr20`.
  postPatch = ''
    cp src/NuGet.config NuGet.config
  '';

  # Framework-dependent: a self-contained publish would duplicate the runtime
  # per project and defeats the shared `$out/lib/whisparr-eros` layout the
  # runtime assembly probing depends on.
  selfContainedBuild = false;

  # `Directory.Build.props` turns on StyleCop plus `AnalysisLevel 6.0-all` with
  # `TreatWarningsAsErrors`. That is a CI gate for upstream, not a build
  # requirement, and it makes the build hostage to the exact analyzer/SDK patch
  # we happen to have. Turning it off also drops the analyzer packages from the
  # restore graph.
  dotnetFlags = [
    "-p:AssemblyVersion=${assemblyVersion}"
    "-p:EnableAnalyzers=false"
    "-p:TreatWarningsAsErrors=false"
  ];

  # Every project uses `<TargetFrameworks>` (plural) even though the list holds
  # a single entry, which msbuild treats as cross-targeting: `dotnet publish`
  # then refuses with NETSDK1129 unless a framework is named. Restore and build
  # are happy either way, so pin it on the install step only.
  dotnetInstallFlags = [ "-p:TargetFramework=net8.0" ];

  executables = [ "Whisparr" ];

  # Whisparr resolves its web assets as `<startup folder>/UI`, where the startup
  # folder is the directory holding `Whisparr.Common.dll`
  # (`AppFolderInfo.StartUpFolder`). That is `$out/lib/whisparr-eros` here.
  postInstall = ''
    cp -r ${frontend} $out/lib/whisparr-eros/UI
  '';

  # `AssemblyLoader.LoadNativeLib` installs a DllImport resolver that maps
  # `sqlite3` to `libsqlite3.so.0`, so SQLite has to be findable on
  # LD_LIBRARY_PATH or the app dies before the first migration. openssl and zlib
  # back the runtime's TLS and compression natives. ICU is already added by
  # buildDotnetModule, and libmediainfo/libcurl are *not* needed: eros probes
  # files with the bundled Servarr.FFprobe and talks HTTP through the managed
  # SocketsHttpHandler (there is no MediaInfo `DllImport` anywhere in the tree).
  runtimeDeps = [
    openssl
    sqlite
    zlib
  ];

  passthru.ui = frontend;

  meta = {
    description = "Adult movie collection manager for Usenet and BitTorrent users (eros/V3 line)";
    homepage = "https://github.com/Whisparr/Whisparr";
    license = lib.licenses.gpl3Only;
    mainProgram = "Whisparr";
    platforms = lib.platforms.linux;
  };
}
