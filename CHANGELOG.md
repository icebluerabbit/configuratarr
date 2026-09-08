# Changelog

## [0.2.0](https://github.com/icebluerabbit/configuratarr/compare/v0.1.2...v0.2.0) (2026-09-08)


### Features

* add komga-v1, bindery-v1 and audiobookshelf-v1 services ([c0b4a26](https://github.com/icebluerabbit/configuratarr/commit/c0b4a26c1ad633ff40c98f57f7e5acdb0666dc1c))
* **whisparr-v3:** add the Whisparr (eros) service ([b8065c8](https://github.com/icebluerabbit/configuratarr/commit/b8065c865d7d46afaae46d9a39bc30071fd59c13))


### Bug Fixes

* **autobrr-v1:** send the feed id in the update body ([c41b0ed](https://github.com/icebluerabbit/configuratarr/commit/c41b0ed5db39abccbf940124b4c0d4a1a326310c))
* **radarr-v3:** type telegram topic_id as an integer ([d4880ec](https://github.com/icebluerabbit/configuratarr/commit/d4880ec05721ad68598591c8cc95f9ccc9ae92ba))

## [0.1.2](https://github.com/icebluerabbit/configuratarr/compare/v0.1.1...v0.1.2) (2026-08-14)


### Bug Fixes

* **core:** resolve refs to not-yet-created custom-sync resources ([c785cd8](https://github.com/icebluerabbit/configuratarr/commit/c785cd888ddc4fbd1181d90b5ddcba4fb2f83265))
* **nix:** drop a stray lazylibrarian arg from the cleanuparr e2e call ([52ebc4e](https://github.com/icebluerabbit/configuratarr/commit/52ebc4e31c009963d2d69d0d5c4b36d0d63a1b76))
* **nix:** stop the lazylibrarian e2e shell leaking its relaunch loop ([bc85efb](https://github.com/icebluerabbit/configuratarr/commit/bc85efbdf938347110ab5d1a4622a9caca9556ac))

## [0.1.1](https://github.com/icebluerabbit/configuratarr/compare/v0.1.0...v0.1.1) (2026-06-16)


### Bug Fixes

* add aarch64-linux to extra-platforms in release-please workflow ([e90d322](https://github.com/icebluerabbit/configuratarr/commit/e90d32200bb4f92b41c38e769d74759cd69aa6ed))

## 0.1.0 (2026-06-16)


### Features

* add Cachix binary cache config to flake.nix ([3a40dfa](https://github.com/icebluerabbit/configuratarr/commit/3a40dfaecd0f1d693b733a531822ef0080e10a3a))
* configure release-please with multi-platform assets build ([f978d68](https://github.com/icebluerabbit/configuratarr/commit/f978d68fdba2f2c5b7f59262ba1a8c58f1dc5095))
* implement structured submodules for options and apply nixfmt ([9a3a119](https://github.com/icebluerabbit/configuratarr/commit/9a3a11936230e52306a0efe3115b9a65194b1efb))
