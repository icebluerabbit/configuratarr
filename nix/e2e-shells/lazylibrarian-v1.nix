# Local e2e dev shell for lazylibrarian-v1.
# Builds LazyLibrarian from source (not in nixpkgs; inlined below), starts it in a
# temp data dir with a pre-seeded config.ini (API enabled + a fixed 32-char API
# key, the length LazyLibrarian requires), waits on the authenticated
# `/api?cmd=getVersion`, and exports LAZYLIBRARIAN_URL + LAZYLIBRARIAN_API_KEY.
#
# Best-effort: if it fails to start, check the log path printed below (likely a
# missing Python dependency — add it to `pyEnv`).
{
  pkgs,
  e2eShell,
  common,
}:
let
  lazylibrarian =
    let
      pyEnv = pkgs.python3.withPackages (
        ps: with ps; [
          cherrypy
          cherrypy-cors
          cheroot
          portend
          python-dateutil
          requests
          urllib3
          pillow
          apscheduler
          six
          mako
          beautifulsoup4
          rapidfuzz
          html5lib
          webencodings
          httplib2
          lxml
          pypdf
          chardet
          charset-normalizer
          pysocks
          python-magic
          xmltodict
          pyyaml
          markdown
          pygments
          irc
          tzlocal
          pytz
          cryptography
          pyopenssl
          oauthlib
          requests-oauthlib
          deluge-client
          apprise
          httpx
        ]
      );
    in
    pkgs.stdenvNoCC.mkDerivation {
      pname = "lazylibrarian";
      version = "unstable-2026-07-10";
      src = pkgs.fetchFromGitLab {
        owner = "LazyLibrarian";
        repo = "LazyLibrarian";
        rev = "40a389ea9f354dc20d8aac2a07e4a7d05b348783";
        sha256 = "186w83nj4c4r0ql1wklg90w233m4284ry50isa28gh1lp8pvdh3m";
      };
      nativeBuildInputs = [ pkgs.makeWrapper ];
      installPhase = ''
        runHook preInstall
        mkdir -p $out/share/lazylibrarian $out/bin
        cp -r . $out/share/lazylibrarian
        makeWrapper ${pyEnv}/bin/python3 $out/bin/lazylibrarian \
          --add-flags "$out/share/lazylibrarian/LazyLibrarian.py"
        runHook postInstall
      '';
    };
  apiKey = "configuratarre2e0000000000000000"; # exactly 32 chars
in
pkgs.mkShell {
  inputsFrom = [ e2eShell ];
  packages = [
    lazylibrarian
    pkgs.curl
  ];
  shellHook = ''
    echo "=== Configuratarr E2E DevShell (lazylibrarian-v1) ==="
    ${common}

    _LL_DATA=$(mktemp -d -t configuratarr-lazylibrarian-XXXXXX)
    cat > "$_LL_DATA/config.ini" <<'EOF'
    [General]
    http_host = 127.0.0.1
    http_port = 5299
    http_root = /
    launch_browser = 0
    api_enabled = 1
    api_key = configuratarre2e0000000000000000
    EOF

    if ! e2e_reclaim_port 5299 LazyLibrarian; then
      return 2>/dev/null || exit 1
    fi

    echo "  starting lazylibrarian..."
    # LazyLibrarian self-restarts once on first run (the launcher normally
    # re-execs; run directly it just exits), so keep relaunching it.
    ( for _n in 1 2 3 4 5; do
        lazylibrarian --datadir "$_LL_DATA" --config "$_LL_DATA/config.ini" \
          --port 5299 --nolaunch >> "$_LL_DATA/lazylibrarian.log" 2>&1
        sleep 1
      done ) &
    _LL_PID=$!

    _ll_wait() {
      local i=0
      while [ $i -lt 90 ]; do
        curl -sf "http://localhost:5299/api?cmd=getVersion&apikey=${apiKey}" > /dev/null 2>&1 && return 0
        sleep 1; i=$((i + 1))
      done
      return 1
    }

    if _ll_wait; then
      export LAZYLIBRARIAN_URL="http://localhost:5299"
      export LAZYLIBRARIAN_API_KEY="${apiKey}"
      echo "  lazylibrarian ready — $LAZYLIBRARIAN_URL"
      echo ""
      echo "  cargo nextest run -p lazylibrarian-v1 --test e2e --run-ignored all -j1"
    else
      echo "  lazylibrarian failed to start — check $_LL_DATA/lazylibrarian.log"
      kill "$_LL_PID" 2>/dev/null
    fi

    _ll_cleanup() {
      kill "$_LL_PID" 2>/dev/null
      wait "$_LL_PID" 2>/dev/null
      rm -rf "$_LL_DATA"
    }
    trap _ll_cleanup EXIT
  '';
}
