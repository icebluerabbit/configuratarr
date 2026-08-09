# nixosTest for lazylibrarian-v1. Called with the pre-built e2e test binary.
#
# LazyLibrarian is **not** in nixpkgs (no package, no NixOS module), so this
# builds it from source (inline below) and runs it as an inline systemd service
# in the test VM, with a pre-seeded config.ini (API enabled + a fixed 32-char API
# key). Best-effort — if it fails to start, a Python dep is likely missing from
# `pyEnv`; add it.
{ pkgs }:
e2eBin:
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
      meta = {
        description = "LazyLibrarian (from source; not packaged in nixpkgs)";
        homepage = "https://gitlab.com/LazyLibrarian/LazyLibrarian";
        license = pkgs.lib.licenses.gpl3Only;
      };
    };
  apiKey = "configuratarre2e0000000000000000"; # exactly 32 chars
  mockAuthor = "Terry Pratchett";
  # Stub the OpenLibrary book API so `addAuthor` resolves offline (the CI VM has
  # no internet). Serves just what LazyLibrarian's OL `find_author_id` +
  # `get_author_info` need — an author search hit and the author record — with a
  # `{}` catch-all so any other call is a harmless empty result.
  bookApiMock = pkgs.writeText "book-api-mock.py" ''
    import json
    from http.server import BaseHTTPRequestHandler, HTTPServer

    AUTHOR = "${mockAuthor}"
    KEY = "OL_TP_A"

    class H(BaseHTTPRequestHandler):
        def do_GET(self):
            if self.path.startswith("/search.json"):
                body = {"numFound": 1, "docs": [
                    {"author_name": [AUTHOR], "author_key": [KEY],
                     "title": "Placeholder", "key": "/works/OL_TP_W"}]}
            elif self.path.startswith("/authors/"):
                body = {"key": "/authors/" + KEY, "name": AUTHOR,
                        "type": {"key": "/type/author"}}
            else:
                body = {}
            data = json.dumps(body).encode()
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(data)))
            self.end_headers()
            self.wfile.write(data)

        def log_message(self, *a):
            pass

    HTTPServer(("127.0.0.1", 8080), H).serve_forever()
  '';
  configIni = pkgs.writeText "lazylibrarian-config.ini" ''
    [General]
    http_host = 127.0.0.1
    http_port = 5299
    http_root = /
    launch_browser = 0
    api_enabled = 1
    api_key = ${apiKey}
    book_api = OpenLibrary
    ol_url = http://127.0.0.1:8080
  '';
in
pkgs.testers.nixosTest {
  name = "lazylibrarian-v1-e2e";
  nodes.machine = {
    systemd.services.book-api-mock = {
      description = "OpenLibrary book-API stub for LazyLibrarian e2e";
      wantedBy = [ "multi-user.target" ];
      before = [ "lazylibrarian.service" ];
      serviceConfig = {
        ExecStart = "${pkgs.python3}/bin/python3 ${bookApiMock}";
        DynamicUser = true;
        Restart = "always";
        RestartSec = 1;
      };
    };
    systemd.services.lazylibrarian = {
      description = "LazyLibrarian";
      wantedBy = [ "multi-user.target" ];
      preStart = ''
        install -Dm600 ${configIni} /var/lib/lazylibrarian/config.ini
      '';
      # LazyLibrarian self-restarts once on first run (the launcher re-execs; run
      # directly under systemd it just exits), so let systemd bring it back —
      # otherwise the service dies and every test fails. No start-limit so the
      # first-run restart never trips it.
      startLimitIntervalSec = 0;
      serviceConfig = {
        ExecStart = ''
          ${lazylibrarian}/bin/lazylibrarian \
            --datadir /var/lib/lazylibrarian \
            --config /var/lib/lazylibrarian/config.ini \
            --port 5299 --nolaunch
        '';
        StateDirectory = "lazylibrarian";
        DynamicUser = true;
        Restart = "always";
        RestartSec = 1;
      };
    };
    environment.systemPackages = [
      e2eBin
      pkgs.curl
    ];
  };
  testScript = ''
    machine.wait_for_unit("book-api-mock.service")
    machine.wait_for_unit("lazylibrarian.service")
    machine.wait_for_open_port(5299, timeout=180)
    machine.wait_until_succeeds(
      "curl -sf 'http://localhost:5299/api?cmd=getVersion&apikey=${apiKey}'",
      timeout=120,
    )
    # Let LazyLibrarian's first-run self-restart settle, then confirm it is stably
    # up before running the suite (so a request doesn't hit it mid-restart).
    machine.sleep(20)
    machine.wait_until_succeeds(
      "curl -sf 'http://localhost:5299/api?cmd=getVersion&apikey=${apiKey}'",
      timeout=120,
    )

    machine.succeed(
      "LAZYLIBRARIAN_URL=http://localhost:5299 LAZYLIBRARIAN_API_KEY=${apiKey} "
      # --test-threads=1: e2e tests share one live instance; run serially.
      "lazylibrarian-v1-e2e --include-ignored --test-threads=1 2>&1"
    )
  '';
}
