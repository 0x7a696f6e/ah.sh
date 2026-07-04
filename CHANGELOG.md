# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0](https://github.com/z1-0/ah.sh/compare/ah-sh-v0.1.1...ah-sh-v0.2.0) (2026-07-04)


### Features

* add ah init command ([dc0aa5e](https://github.com/z1-0/ah.sh/commit/dc0aa5e8c967688d5ce98871ed54ae78f2bb8fd6))
* add ah update command ([6a10e5d](https://github.com/z1-0/ah.sh/commit/6a10e5d863f528807e4b594b15fa34227a1ed8a6))
* add AppConfig struct and load_config() ([22fce7c](https://github.com/z1-0/ah.sh/commit/22fce7ce30353bb668737f7441b59ae8fac35d8d))
* add cachix workflow for binary caching ([a4beea9](https://github.com/z1-0/ah.sh/commit/a4beea9656bfd0bbe2062ff5cb6d30b814d245da))
* add clap_complete dependency ([a6d3cc1](https://github.com/z1-0/ah.sh/commit/a6d3cc1b085bbf1f15e91f6f964c79e37822e651))
* add config module with get_config_path() ([3315862](https://github.com/z1-0/ah.sh/commit/3315862ac95d291e91cfb86683cf2f79ac1167cc))
* add custom shell detection with libc integration ([3b0bd3a](https://github.com/z1-0/ah.sh/commit/3b0bd3a6542c126ff642480ca9fbd171574ff2f5))
* add dynamic shell completions for languages and session keys ([c8d39b9](https://github.com/z1-0/ah.sh/commit/c8d39b9f7b252ca42477cf3e1642ac7d53f46066))
* add HistoryEntry type for session history ([1ba25ee](https://github.com/z1-0/ah.sh/commit/1ba25ee9c359e2c950ffd882451904a77998ba43))
* add Init and Update command placeholders ([e5f5c4b](https://github.com/z1-0/ah.sh/commit/e5f5c4b519e8e5845c7fd7f8412f458330357b7a))
* add language help text to session completion candidates ([75f0806](https://github.com/z1-0/ah.sh/commit/75f080671b550ea1e5e810e4b658c1cd52b4d561))
* add language normalization layer and cli provider support ([aec0746](https://github.com/z1-0/ah.sh/commit/aec0746292393357a4d0abb9c17d2fcd35669499))
* add last_used_at and last_updated_at to session metadata ([193b772](https://github.com/z1-0/ah.sh/commit/193b7723ad9b88ee6754fc1511eb7cd87830fe2a))
* add logging infrastructure with tracing subscriber ([2db667c](https://github.com/z1-0/ah.sh/commit/2db667cf2023514dd408da9988f28871655097a5))
* add Nix availability check before command dispatch ([5be7307](https://github.com/z1-0/ah.sh/commit/5be73071b28eee04939e1429a541d03c1a554bcd))
* add output beautification with table formatting ([1d753a6](https://github.com/z1-0/ah.sh/commit/1d753a67bbc85312470445f97d17ea6a1ef5196a))
* add print_session_history function ([1c2dd4e](https://github.com/z1-0/ah.sh/commit/1c2dd4e6788276242d6994df9f17da2feb0fb50c))
* add session clear/remove with typed selector and fixed id length ([32cb638](https://github.com/z1-0/ah.sh/commit/32cb6384722cd28ea67f51fefb2e312aa22e0d74))
* add session history prompt in CLI ([0ece41f](https://github.com/z1-0/ah.sh/commit/0ece41f63df56080385422c4e6fa55f03c630e24))
* add shell completion module ([a6d3cc1](https://github.com/z1-0/ah.sh/commit/a6d3cc1b085bbf1f15e91f6f964c79e37822e651))
* add shell completion subcommand ([a6d3cc1](https://github.com/z1-0/ah.sh/commit/a6d3cc1b085bbf1f15e91f6f964c79e37822e651))
* add structured logging with tracing and #[instrument] ([0915a41](https://github.com/z1-0/ah.sh/commit/0915a417f3342e072dd929a24a2079126fbb3a80))
* add top-level update and restore commands as session aliases ([4132bb8](https://github.com/z1-0/ah.sh/commit/4132bb8b4472a9b0219a62a4c821d4c7fb00158d))
* add update_history and find_by_path functions ([6d23ecc](https://github.com/z1-0/ah.sh/commit/6d23ecc6f6d67296fb52ea372aa041d3fb50b405))
* add user configuration support with module refactoring ([a53caf5](https://github.com/z1-0/ah.sh/commit/a53caf57411b2bdb0fc3f66543c5aef48273e8a4))
* bind generated flake.nix to session-specific cache directories ([eb3ff24](https://github.com/z1-0/ah.sh/commit/eb3ff2409720d46a41692f0d05245be68c28eda3))
* **build:** replace fenix with rust-overlay for toolchain ([2aabb6f](https://github.com/z1-0/ah.sh/commit/2aabb6f03c0f07286f5f86594eda80a4c9accfe1))
* cap concurrent dev-template fetch workers ([2f660c2](https://github.com/z1-0/ah.sh/commit/2f660c27ff2258606040d9a00d5d12c86694b15f))
* cap concurrent dev-template fetch workers ([2f660c2](https://github.com/z1-0/ah.sh/commit/2f660c27ff2258606040d9a00d5d12c86694b15f))
* change default provider to dev-templates ([915980b](https://github.com/z1-0/ah.sh/commit/915980bc66edcb19f855462c9af734eeaba1d3d0))
* **cli:** add provider subcommand suite ([105d5e4](https://github.com/z1-0/ah.sh/commit/105d5e4a90d3bf9c332486235271554b7372ba66))
* **cli:** print help on no args and exit 2 ([4b547f0](https://github.com/z1-0/ah.sh/commit/4b547f0999f9dddf520a6668904fac526c7f0dfc))
* **cli:** print help on no args and exit 2 ([68aba7b](https://github.com/z1-0/ah.sh/commit/68aba7b6c4e30993440daf19aeb6bb8f5285c2e2))
* **cli:** rename create to lang command ([59deb4a](https://github.com/z1-0/ah.sh/commit/59deb4a970088953c1accb29e7dc0d68eef56d7a))
* **cli:** rename lang command to use ([ae10006](https://github.com/z1-0/ah.sh/commit/ae100067c4a776fa0ca5a2c265bd2d3b53c154c5))
* decouple provider logic and add multi-provider support ([4cce1de](https://github.com/z1-0/ah.sh/commit/4cce1dece3a126684b08844edac78e6354344c45))
* deduplicate languages before session generation ([a9e26d7](https://github.com/z1-0/ah.sh/commit/a9e26d78cb77a9c9d1dce51e4784bbe01ec6695c))
* **dx:** colorize warnings on tty ([913d117](https://github.com/z1-0/ah.sh/commit/913d117085c16c57ef4b0a36dbfec185c50edbe5))
* export find_by_path and update_history in session service ([8b7ba48](https://github.com/z1-0/ah.sh/commit/8b7ba482da92f391feae57e15f6f304eb6640d4c))
* implement dynamic Nix shell template generation with AST parsing ([411f5cd](https://github.com/z1-0/ah.sh/commit/411f5cde4366b0ce33f21403dd997548bcfe11a7))
* implement dynamic Nix shell template generation with AST parsing for dev-templates ([15fe73d](https://github.com/z1-0/ah.sh/commit/15fe73db83b284e184a6cdc28908566c9a344966))
* implement session management and profile support ([0f611de](https://github.com/z1-0/ah.sh/commit/0f611de919ccbd932d130eaebea72351afde10f5))
* implement static language lists and weekly CI update ([38d3d2f](https://github.com/z1-0/ah.sh/commit/38d3d2fdcc7594bb3948b662e6cde24ada0dbb3d))
* improve session list time display with human-readable relative format ([80e84e7](https://github.com/z1-0/ah.sh/commit/80e84e73522f9082db9118dd6f76742bbf3fb693))
* lock dev-templates flake inputs ([30a56c8](https://github.com/z1-0/ah.sh/commit/30a56c8cc549295c2423b0e98aef3654b614bd0f))
* log nix version on check_nix_available success ([65d7009](https://github.com/z1-0/ah.sh/commit/65d7009774370b322210ff89918a9c184247f93f))
* modify provider table display ([93c25ee](https://github.com/z1-0/ah.sh/commit/93c25ee2d0b6ab4462ad5a127de5c1ac8aa3ba21))
* modify session table display ([8fdfd70](https://github.com/z1-0/ah.sh/commit/8fdfd70a02e5e19baa15689081ebec1ea9b77a8b))
* move schemars to dev-dependencies with cfg_attr(test) ([044498e](https://github.com/z1-0/ah.sh/commit/044498ecde652e420266dc5b4e80e7d05e2f4c97))
* preload config on startup to auto-create default ([c3b4897](https://github.com/z1-0/ah.sh/commit/c3b48970c826767d9b18237f1f08dce634d36a01))
* prevent data corruption via atomic file writes ([dd0f040](https://github.com/z1-0/ah.sh/commit/dd0f040c9d7b7738fef3f577fd6f86747ee40724))
* **provider:** add attrs cache with corruption-safe fallback ([f6e8e66](https://github.com/z1-0/ah.sh/commit/f6e8e66edc97c28f0e76cb71e73e88717022a478))
* **provider:** add injectable store resolver with prefetch fallback ([9d7dbf7](https://github.com/z1-0/ah.sh/commit/9d7dbf7e05f62ffb079d61b89121cee826b19103))
* **provider:** parse dev-templates attrs from nix store with cache-first flow ([c09d939](https://github.com/z1-0/ah.sh/commit/c09d93942d909a54404b4ff11e20d80e567ae5b6))
* record directory history when entering session ([7286b33](https://github.com/z1-0/ah.sh/commit/7286b33fcbb83ba4f38a753d8dab8ef6b166c48d))
* remove formatter/linter provisioning and update AGENTS.md build priority ([753149f](https://github.com/z1-0/ah.sh/commit/753149f63f3ce883e3461adaae8bbbbb48c03368))
* switch to dynamic shell completion ([abf2bfa](https://github.com/z1-0/ah.sh/commit/abf2bfab13b54675f6f6d72adef28db2d5975e07))
* trigger session history from restore command instead of use ([6d16e8b](https://github.com/z1-0/ah.sh/commit/6d16e8bc06bd0afe2efc6b336c76972fe4fdefb1))


### Bug Fixes

* auto-enter dev shell after flake update instead of prompting ([a19f5da](https://github.com/z1-0/ah.sh/commit/a19f5dacbe848d933e28d9c91a2e3046d4a7814c))
* **ci:** exclude CHANGELOG.md from treefmt to prevent release-plz format conflicts ([0b1d899](https://github.com/z1-0/ah.sh/commit/0b1d8993bda783f567ae76b7aa1d4c158a57c506))
* **ci:** modify release-plz token ([74d4759](https://github.com/z1-0/ah.sh/commit/74d4759cbe60a9bc16821e358c6e23f225695fb4))
* clear current session when removing or clearing sessions ([a9aa769](https://github.com/z1-0/ah.sh/commit/a9aa7699a6486262ecc607b149c1274617b8d9f4))
* configure release-plz to handle non-publishable package ([e1f5d34](https://github.com/z1-0/ah.sh/commit/e1f5d344b27e70f6b74d04e77744d5cb9a23cc22))
* correct include_str path and remove JsonSchema dependency ([fa22da7](https://github.com/z1-0/ah.sh/commit/fa22da735190ee7567a0db43e20ceb3843fe36b6))
* correct print_error call with single formatted string argument ([4091d6e](https://github.com/z1-0/ah.sh/commit/4091d6e8be2354861790f3d38961084bea0f1e11))
* determine --profile usage by checking if profile file exists ([4533700](https://github.com/z1-0/ah.sh/commit/45337005c03ff64f30ca54b3448163e822ca2144))
* ensure logs are flushed before process replacement and on exit ([a77fc61](https://github.com/z1-0/ah.sh/commit/a77fc6138590d2aa99982f87db978203477665e5))
* **executor:** correct nix develop argument flow for session lifecycle ([1d57b91](https://github.com/z1-0/ah.sh/commit/1d57b91d134fddf6ef28e22a58e4c7aaf4bac889))
* guard default config write with existence check ([8706365](https://github.com/z1-0/ah.sh/commit/870636564b2f05b76088dc743130d97e601b86d5))
* optimize session operations and remove redundant path computation ([df8f652](https://github.com/z1-0/ah.sh/commit/df8f6521cfe6ac7407313b01b1424ffa4f18321a))
* **paths:** use ProjectDirs data/cache roots directly ([d0ebe7d](https://github.com/z1-0/ah.sh/commit/d0ebe7d6a269972c2f2dd6a768f5b379f4c372f5))
* persist session metadata when reusing flake ([0d97d6f](https://github.com/z1-0/ah.sh/commit/0d97d6fbbe3fbce88753b861147ca5ba08539209))
* properly handle nix flake update errors ([a78568e](https://github.com/z1-0/ah.sh/commit/a78568e94447821183e70d91ef99a0276cf4b205))
* **provider:** degrade cache read errors to warning and miss ([c55b67e](https://github.com/z1-0/ah.sh/commit/c55b67ebcc18b1875c1cd048b6ce2b202f7025c7))
* **provider:** use valid nix flake prefetch fallback command ([06c1cbd](https://github.com/z1-0/ah.sh/commit/06c1cbdbe034f978f1e3f87051e299e147f6019a))
* restore info message when no session history found ([a435e35](https://github.com/z1-0/ah.sh/commit/a435e3534c2af3a817ad98a2360d71c4f685c237))
* restore lost tracing-attributes dependency and sync Cargo.lock ([f74150c](https://github.com/z1-0/ah.sh/commit/f74150c5a240c7bc51b4240ad36a68a9c6bb1135))
* reuse existing session flake without overwrite ([2a3cb87](https://github.com/z1-0/ah.sh/commit/2a3cb87e9c497dd3c91191c5de1be6ff8eaa1303))
* session/mod.rs remove unresolved SessionError import ([84c59e9](https://github.com/z1-0/ah.sh/commit/84c59e9732c551d7039d8b500652dfacd9c3aa90))
* **session:** render inner value in SessionKey Display ([382c406](https://github.com/z1-0/ah.sh/commit/382c4060c608adef5406ef25236c8f9f7c7487b6))
* set initial version to 0.1.0 to enable release-plz release ([6a28aa3](https://github.com/z1-0/ah.sh/commit/6a28aa34b559c8fe4bca0987ccde53b46e7ca0e0))
* translate check_nix_available error message to English ([c86b251](https://github.com/z1-0/ah.sh/commit/c86b251896bddb144e1d4b0d809d4e6c8d9536cb))
* update comment and optimize string allocation in storage ([b6dbea5](https://github.com/z1-0/ah.sh/commit/b6dbea5bab82a5f6b9d57c1209d8ca957efd94e9))
* update is_terminal call to is_interactive in manager.rs ([a1c578a](https://github.com/z1-0/ah.sh/commit/a1c578a3daba256634c8aee885e9ab855b76c451))
* use binary name for app name and align crate name ([730fb0d](https://github.com/z1-0/ah.sh/commit/730fb0d9a5fc1a615e0d1bc71455c2d2d3042b4b))
* use git-only mode in release-plz for non-published package ([be0bbe1](https://github.com/z1-0/ah.sh/commit/be0bbe1192192b6e828e80109c93d09cb6ea233f))
* use HISTORY_LIMIT constant instead of hardcoded 5 ([3e49fa0](https://github.com/z1-0/ah.sh/commit/3e49fa04bdf73bc725e8e23a4a278108cd617258))


### Performance Improvements

* optimize language normalization and session lookup ([c0d91bc](https://github.com/z1-0/ah.sh/commit/c0d91bc0c4d5184bddd8074db8298613a69afa60))


### Code Refactoring

* abstract provider logic into traits and optimize resource path handling ([11bfad4](https://github.com/z1-0/ah.sh/commit/11bfad42a8c2a2281c67a96df40f59fdc3b7d0b4))
* add #[instrument] to manager functions and clean up output ([e8fc024](https://github.com/z1-0/ah.sh/commit/e8fc024b7d8f89be78b712c227a6905de3e3f54d))
* add #[instrument] tracing to cmd, session and dev_templates modules ([e82456c](https://github.com/z1-0/ah.sh/commit/e82456cd7e6ee069267a0e47d7ce78a5e49bce04))
* add anyhow/tracing, remove thiserror ([fa52cc6](https://github.com/z1-0/ah.sh/commit/fa52cc621e99b6f1922301763ceeb0aa6f1cf3e0))
* add get_cwd utility and deduplicate current_dir calls ([28a2d46](https://github.com/z1-0/ah.sh/commit/28a2d46128e717726938667c7030676536af7671))
* add layering module skeleton ([7c59315](https://github.com/z1-0/ah.sh/commit/7c59315544cdf056e3ec6c94ff6bc84f01d2837f))
* add session module types ([c267f0c](https://github.com/z1-0/ah.sh/commit/c267f0cdfc0eb1931fc471cadf6ec1929da12650))
* add session module types ([cddf426](https://github.com/z1-0/ah.sh/commit/cddf426fdbfa48d1d579b5c4c99fa5ed15afe171))
* add session storage ([27b7a3d](https://github.com/z1-0/ah.sh/commit/27b7a3de44a619fa136cdf6e6e2fe2e35488f4c3))
* add session struct ([9af2f76](https://github.com/z1-0/ah.sh/commit/9af2f7642feb68c746ead16e7d32a312b4dfd5b8))
* add structured logging to cmd, session and dev_templates modules ([fd01a91](https://github.com/z1-0/ah.sh/commit/fd01a915d748acffaa22346481af5c9a7f502ddd))
* add tracing init in main.rs ([514bb4a](https://github.com/z1-0/ah.sh/commit/514bb4a387800b904d9d02f663b6a4597d45d2c0))
* align session module exports ([50a8c12](https://github.com/z1-0/ah.sh/commit/50a8c12a5ab21af9a3ef151566f235232e099994))
* **app:** restrict SessionApp visibility ([99b3f98](https://github.com/z1-0/ah.sh/commit/99b3f989e43ca8b7bb2d78e60318a053b30a143e))
* **app:** route session create through SessionApp ([bd84ad1](https://github.com/z1-0/ah.sh/commit/bd84ad141c545aa5e1c56a80a143f9385c37080a))
* **app:** route session list through SessionApp ([c5fb17d](https://github.com/z1-0/ah.sh/commit/c5fb17da0a247ceeab554712a8c48df1b1eaf5ff))
* **app:** route session remove/clear through SessionApp ([6937d73](https://github.com/z1-0/ah.sh/commit/6937d73669817a217e133c3358fa90d4b7de71e6))
* **app:** route session restore through SessionApp ([974ae01](https://github.com/z1-0/ah.sh/commit/974ae012566db95f5e2a6dc21eaf35b619495b95))
* centralize session file/directory names in paths.rs ([47f6cc3](https://github.com/z1-0/ah.sh/commit/47f6cc3de89ce5e038e3758f75dde9983bc1c1ae))
* **ci:** migrate from release-plz to release-please ([23f63d4](https://github.com/z1-0/ah.sh/commit/23f63d4adddd2188b4a9772d23992eb63a2270db))
* clean up fs-err usage, fix TOCTOU and deduplicate history logic ([5ea0b7b](https://github.com/z1-0/ah.sh/commit/5ea0b7bc35d6009b966ed85a9f5b7e1f2724691b))
* clean up logging implementation issues ([75782ea](https://github.com/z1-0/ah.sh/commit/75782ea2be91c972a899c944f41e2a2f11cf9821))
* clean up utils.rs and optimize buffer size ([4018ec8](https://github.com/z1-0/ah.sh/commit/4018ec816d9bee68d415ac402f7c097dc8c4abca))
* cleanup language_maps.rs ([133344f](https://github.com/z1-0/ah.sh/commit/133344f321d2019bea0d4bc7a18491a0fc7f8397))
* **cli:** refactor(cli):  ([6c09146](https://github.com/z1-0/ah.sh/commit/6c09146bcd3d2f5ad2ed568a7b6f532e16a57f76))
* cli/mod.rs use anyhow ([5417fcc](https://github.com/z1-0/ah.sh/commit/5417fcca3522c358ee97c69af9668a69297c1bd3))
* **cli:** avoid unreachable when session restore diverges ([9e439e7](https://github.com/z1-0/ah.sh/commit/9e439e7b58b9a6a03fa5b03b812ac516fe270071))
* cmd.rs use anyhow, replace eprintln with tracing ([c74c830](https://github.com/z1-0/ah.sh/commit/c74c830207b501c31a9c5817e7277d30e69fb995))
* **cmd:** centralize command execution and errors ([eac744e](https://github.com/z1-0/ah.sh/commit/eac744e0748fdb5c23cce6dc0f55f511931fd471))
* **cmd:** rename executor module ([144715b](https://github.com/z1-0/ah.sh/commit/144715b445f6d14c058c2d8066788f1be15ed037))
* code cleanup and formatting improvements ([1a9dadc](https://github.com/z1-0/ah.sh/commit/1a9dadc8f8a327e8afe48f5532ee4ecf96c7f0f4))
* combine nested if-let conditions with && operator ([7117302](https://github.com/z1-0/ah.sh/commit/71173027b0f9e16c03240a88660609eb4bf09655))
* complete std::fs to fs-err migration and clean context ([edc7759](https://github.com/z1-0/ah.sh/commit/edc77595b6743d1ad9dafbcca220e421a8becff3))
* consolidate language mapping functions and remove duplication ([ad06c65](https://github.com/z1-0/ah.sh/commit/ad06c657f3d48cce91872ca03de90e4f232053a0))
* consolidate provider module and rename types ([2307bf2](https://github.com/z1-0/ah.sh/commit/2307bf2aaaa82e56338033287e10afb1c08027d0))
* deduplicate parent-dir creation and simplify error handling ([90e4990](https://github.com/z1-0/ah.sh/commit/90e4990e661b6f5b34bd2727b1a53febbdc75dc6))
* delete error.rs and warning.rs - using anyhow + tracing instead ([e17bdaa](https://github.com/z1-0/ah.sh/commit/e17bdaaac80b2511666393d0aac204262be33768))
* **dev_templates:** inline store_resolver functions and simplify prefetch ([a3dab32](https://github.com/z1-0/ah.sh/commit/a3dab3216933dd53bc357e1a345681cb7295d90e))
* **dev_templates:** replace manual thread pool with rayon ([0ff17f9](https://github.com/z1-0/ah.sh/commit/0ff17f9fe45bcc55b83f9be737ada85c798ad31e))
* **dev_templates:** simplify store_resolver by removing over-engineered abstraction ([0be1de5](https://github.com/z1-0/ah.sh/commit/0be1de5dbeaa82d081defddcf370502b5f5737f7))
* **dev_templates:** single prefetch for dev-templates repo ([c47c070](https://github.com/z1-0/ah.sh/commit/c47c07056f7be5797c890a27b26374cb92b213bc))
* **dx:** avoid cloning warnings when printing ([564476a](https://github.com/z1-0/ah.sh/commit/564476a03084d10225ef52b769709790d4ed0155))
* eliminate boolean parameter smell and strengthen type safety ([2cb874b](https://github.com/z1-0/ah.sh/commit/2cb874bb12367ae67da207b54eca629ac4cdc13c))
* embed provider templates as assets and sync to ~/.local/share/ah at runtime ([3842a69](https://github.com/z1-0/ah.sh/commit/3842a69a9a2b067f9bba385888ac46121e55fafd))
* **executor:** return diverging Result and surface exec errors ([2666825](https://github.com/z1-0/ah.sh/commit/2666825208c3441c8382055745550e69e1041d64))
* extract HISTORY_LIMIT constant for session history ([265c0c5](https://github.com/z1-0/ah.sh/commit/265c0c519f97ec5673fa783c86adefe1176b7f3d))
* extract LogLevel enum and integrate into config ([5967ec2](https://github.com/z1-0/ah.sh/commit/5967ec214a63a696b3ccf2574579c06d682018dd))
* extract provider detection logic in implicit use ([1905b03](https://github.com/z1-0/ah.sh/commit/1905b03796bcc8f4a7b8f2081647469bcc249ea5))
* extract provider types to dedicated types.rs module ([752d376](https://github.com/z1-0/ah.sh/commit/752d376a3acf064f05f93c29b00c543f1145b3f6))
* extract session lookup functions to storage module ([8b97360](https://github.com/z1-0/ah.sh/commit/8b97360bb9eaa1e8f66a6bca2ff2eb71139efe6d))
* extract session lookup helpers and simplify remove_sessions ([1db869e](https://github.com/z1-0/ah.sh/commit/1db869eb709dc147cebefa26d6e970f985bf988f))
* extract session reading logic and use FP style ([032db10](https://github.com/z1-0/ah.sh/commit/032db10667385c63c0f4b59e32a6336572a02061))
* extract shared nix develop command setup ([0dac508](https://github.com/z1-0/ah.sh/commit/0dac508c5339564c9e20385ea028c8e5f52510d6))
* finalize session module structure ([a13cc8f](https://github.com/z1-0/ah.sh/commit/a13cc8f4c6eec5fcfe770898ddd22e97a1398339))
* HashMap capacity pre-allocation and clean up provider list ([6b8e75f](https://github.com/z1-0/ah.sh/commit/6b8e75f5b5e435d8249640e079a78066b7b80afa))
* implement senior-architect-grade refactoring (custom errors, DRY providers, and decoupled CLI) ([eff1093](https://github.com/z1-0/ah.sh/commit/eff1093e15bf9fddd936e30dcd08808d359c85a1))
* improve build error messages with specific context ([d7bef3a](https://github.com/z1-0/ah.sh/commit/d7bef3aa90ef061345b18bcf2630b46922ecb540))
* improve CLI help text for better readability ([070c499](https://github.com/z1-0/ah.sh/commit/070c499d06479f789e62cea02509830d8dfad4f4))
* improve error handling and simplify current session cleanup ([b8dc8af](https://github.com/z1-0/ah.sh/commit/b8dc8af282682dbcb8c84207b5fad9879c5c8543))
* improve error handling in get_nix_store_path ([66306be](https://github.com/z1-0/ah.sh/commit/66306be3f5be6586eeaa9a5593eef77fe6e76e99))
* inline build_nix_develop_cmd into nix_develop_of_session ([ed36567](https://github.com/z1-0/ah.sh/commit/ed36567446751444f015d2a8c733a666e22e871e))
* inline exec/run in cmd.rs and simplify error messages ([4edef93](https://github.com/z1-0/ah.sh/commit/4edef936bc1f0b7a091034cfcface9f9054f5c9b))
* inline read_session_from_path and update return types ([930d264](https://github.com/z1-0/ah.sh/commit/930d2648a1b35aa9411c4c167c9bc38ca414a585))
* integrate LogLevel enum into log filtering with release profile ([ff391de](https://github.com/z1-0/ah.sh/commit/ff391de4f341407c97badd7c4fde412702cb45e3))
* introduce domain-specific type aliases and consolidate language handling ([8550140](https://github.com/z1-0/ah.sh/commit/8550140f591d4c25f16f4f7e9f02c39c07ddda99))
* language module and centralize language normalization ([ac5368d](https://github.com/z1-0/ah.sh/commit/ac5368db83e7abeb23951e165cb168b1bcd4a8e0))
* manager.rs use tracing for logs, keep println for output ([e30d9c5](https://github.com/z1-0/ah.sh/commit/e30d9c5c64b76513e2461e84f1a3a0b3ff14b351))
* migrate from console to crossterm ([2f722ff](https://github.com/z1-0/ah.sh/commit/2f722ff31d0649b39adcbf7a54eb63d4b09dac1d))
* migrate OnceLock to LazyLock and simplify provider initialization ([899637c](https://github.com/z1-0/ah.sh/commit/899637c8cab7f24f656fb5463c79975fdf45833f))
* migrate to fs-err and clean context ([c4c1fa3](https://github.com/z1-0/ah.sh/commit/c4c1fa347407309f878fb5f92cb57d6cd25d6324))
* migrate to fs-err and clean context ([7a4836c](https://github.com/z1-0/ah.sh/commit/7a4836c4de1eaf74faa6fd89341e6557d7c61dc2))
* move --provider to use subcommand and implement robust implicit use induction ([6c09146](https://github.com/z1-0/ah.sh/commit/6c09146bcd3d2f5ad2ed568a7b6f532e16a57f76))
* move non-storage logic to mod.rs and improve naming ([8fbb173](https://github.com/z1-0/ah.sh/commit/8fbb1731abb0b322ce6f57735104694c42ac976b))
* move Provider loading logic to provider module ([7468416](https://github.com/z1-0/ah.sh/commit/7468416c17280a0f0166149e22371408e5e309b8))
* move session service ([bbb528b](https://github.com/z1-0/ah.sh/commit/bbb528bcaef9d40d80e7ed1c48e2ec835af7b743))
* move SessionError into sessions module ([b9a0751](https://github.com/z1-0/ah.sh/commit/b9a075147985da24bb40111bcd3f93fd682a8b7c))
* optimize lookup logic and improve error context ([e98ccde](https://github.com/z1-0/ah.sh/commit/e98ccdee8043543fbffc15760288f4020c710278))
* optimize project structure and improve separation of concerns ([b5cf2dd](https://github.com/z1-0/ah.sh/commit/b5cf2dd7da37d60695f3e54bf0b3c2fd52504e17))
* optimize session lookups and remove unused find_in_list ([a3396d7](https://github.com/z1-0/ah.sh/commit/a3396d770171cc8b382c432e684f278612c7a97d))
* paths.rs use anyhow ([338aedc](https://github.com/z1-0/ah.sh/commit/338aedc183e9e31a4edcf0630425263d8106c96a))
* **paths:** move session dir helper to paths module ([ccf935e](https://github.com/z1-0/ah.sh/commit/ccf935e304ded6fd96c68e6ec3a4bd16f32cc29d))
* **paths:** use directories crate for xdg paths ([d18563f](https://github.com/z1-0/ah.sh/commit/d18563f9cfd591caf75e2d6346a10450a9628f27))
* plumb structured warnings and remove library eprintln ([fe7dc1d](https://github.com/z1-0/ah.sh/commit/fe7dc1d1be2fe8e35b28542c124f67e32be739f8))
* propagate config errors instead of inline process::exit ([9975da5](https://github.com/z1-0/ah.sh/commit/9975da5b43409fddec883ebcac2fac2c9541e78e))
* provider/dev_templates/mod.rs use anyhow + tracing ([8278365](https://github.com/z1-0/ah.sh/commit/827836536ad6c6323e13ba8e082333d5afc06819))
* provider/devenv/mod.rs use anyhow ([3e74062](https://github.com/z1-0/ah.sh/commit/3e74062cc7d3b8a0f4c9988ed80d15ac8a2308dd))
* provider/language_maps.rs use anyhow ([13306bf](https://github.com/z1-0/ah.sh/commit/13306bf70ecb96b9146274b650c068a0f98b12f7))
* provider/registry.rs use anyhow ([5257c42](https://github.com/z1-0/ah.sh/commit/5257c42de4529612b7c3ab3176870b4aa2eb3229))
* provider/types.rs remove AppWarning, use anyhow ([76aa8d9](https://github.com/z1-0/ah.sh/commit/76aa8d9972d8b5856412102094f856b9aa0239d2))
* **provider:** centralize attrs cache path and remove dev-templates tests ([7cc31ef](https://github.com/z1-0/ah.sh/commit/7cc31ef24f22d8ffe48e739550f3afdbb7f9a524))
* **provider:** format gen file ([f66186d](https://github.com/z1-0/ah.sh/commit/f66186d5b913a98756f331af214a1970ee7ed5cb))
* **provider:** remove legacy dev-templates fetcher path ([91571dd](https://github.com/z1-0/ah.sh/commit/91571ddd225c6ff40ad6611253a0dd8a2c0bf437))
* **provider:** remove unused registry helpers ([3b29e49](https://github.com/z1-0/ah.sh/commit/3b29e49a89b0d0ae3a105f739a96a981d001c39f))
* **providers:** consolidate language map handling ([7d11fa2](https://github.com/z1-0/ah.sh/commit/7d11fa2a468fcb6d59bcb70cbc74c5e0017c3909))
* **provider:** separate metadata from shell execution ([f19e18b](https://github.com/z1-0/ah.sh/commit/f19e18b84671fdf8e6e5f5b4a009a9d306f23116))
* **providers:** fail-fast on invalid language aliases ([b20f896](https://github.com/z1-0/ah.sh/commit/b20f89668164cceadf494611331c6fca89dced25))
* **providers:** make language normalization fallible ([3e4936e](https://github.com/z1-0/ah.sh/commit/3e4936ee2deb34eb1cc26ed2ad1a3be0fa6fbad0))
* **providers:** move language mappings per provider ([0fae37b](https://github.com/z1-0/ah.sh/commit/0fae37b861212469c65c735d67edca97e28985fc))
* **provider:** streamline dev-templates prefetch parsing ([3e22e8b](https://github.com/z1-0/ah.sh/commit/3e22e8b8254f290678d8bd4f5c895b5dd9a05c6b))
* **provider:** switch dev-templates pipeline to lock-probe + store resolver ([d4c8463](https://github.com/z1-0/ah.sh/commit/d4c8463c367b2a14531af2462e32bdc37385124a))
* reduce session setup overhead and lint noise ([2f47550](https://github.com/z1-0/ah.sh/commit/2f47550be5427226c04369e3645354e6882681df))
* remove chrono dependency and simplify history.json ([61dd982](https://github.com/z1-0/ah.sh/commit/61dd9824e9f0aed2b8dfb816509909002a5be4f7))
* remove duplicate exec function, reuse from cmd module ([f857335](https://github.com/z1-0/ah.sh/commit/f8573358f090c67f4f302473e42b76d4dc2b42e5))
* remove EnsureFilesResult from provider flow ([fd5b833](https://github.com/z1-0/ah.sh/commit/fd5b833928f8bfeae622723b02e17a14ab006a93))
* remove error/warning module exports from lib.rs ([c299563](https://github.com/z1-0/ah.sh/commit/c2995635879fca1e9d4ca51e9034ba57be766622))
* remove hardcoded exit-code string matching ([dcb831d](https://github.com/z1-0/ah.sh/commit/dcb831d8d03bbdf705884a0d69ed0348cf3d1ddd))
* remove implicit use command and simplify CLI structure ([d535e1c](https://github.com/z1-0/ah.sh/commit/d535e1c242bc36b93dc0d3b14a70e98409d22371))
* remove init command and fix crate name ([5596828](https://github.com/z1-0/ah.sh/commit/5596828afff70d322cb79efa552a3529b21d8e57))
* remove ProviderAssetManager and simplify provider architecture ([ce3c8ed](https://github.com/z1-0/ah.sh/commit/ce3c8edae373129e18acc13ebe2c04d360c35dc8))
* remove ProviderShowSelector, use ProviderType directly ([e9096e2](https://github.com/z1-0/ah.sh/commit/e9096e21f70e2127a224f2f8f19cea3233572a77))
* remove redundant check_nix_available calls ([c84b041](https://github.com/z1-0/ah.sh/commit/c84b0413a7f5807d5249a1a3b6ba23f8a75c5434))
* remove redundant comments ([d44139a](https://github.com/z1-0/ah.sh/commit/d44139aca62d0273fa9f18bb100a409d5507ef9f))
* remove redundant error context messages and comments ([9c29622](https://github.com/z1-0/ah.sh/commit/9c2962290aa997579add78a5558e2a0fb06f89f3))
* remove service layer in session module ([c335605](https://github.com/z1-0/ah.sh/commit/c335605880a399d964963d6a892c0095bdba7fb2))
* remove session_dir field and simplify control flow ([533c621](https://github.com/z1-0/ah.sh/commit/533c6213ad774a9040004d1d291b45083793b2e8))
* remove tracing dependency and use print/eprintln for CLI output ([d3fbb80](https://github.com/z1-0/ah.sh/commit/d3fbb800ce78c51fb6250e701c5c00ea7ff0b010))
* remove unused code and add provider alias cache ([e44d0f4](https://github.com/z1-0/ah.sh/commit/e44d0f4ad98535773219748fa1a028c4494cf258))
* remove unused use_profile parameter from nix_develop_of_session ([8d3470e](https://github.com/z1-0/ah.sh/commit/8d3470eb6298c118a2133b1966050a2ebf89e5ae))
* rename AhError to AppError ([81702d2](https://github.com/z1-0/ah.sh/commit/81702d2ee8433e6e66aca1c845f6c51b33cf6fb8))
* rename path constants for clarity ([6ebb009](https://github.com/z1-0/ah.sh/commit/6ebb009ecb0cf7e7aacad0bbf8f57105bdb2d879))
* rename providers module to provider ([db5571e](https://github.com/z1-0/ah.sh/commit/db5571e312e7659c1a7e644d205cf6283c9cd1c3))
* rename selector var ([cfb4060](https://github.com/z1-0/ah.sh/commit/cfb40601b9960842cb17bb923f8be4e12c5c1d71))
* rename session lookup functions for consistency ([b1cad93](https://github.com/z1-0/ah.sh/commit/b1cad938eff4d2b2fd3dede2b3dd765039c7fb36))
* rename stderr_layer to console_layer ([5ad18eb](https://github.com/z1-0/ah.sh/commit/5ad18ebe56178e12e78609bd3f3040841fcfec6c))
* rename to path.rs and optimize module structure ([4da5a82](https://github.com/z1-0/ah.sh/commit/4da5a8202d2bc76086add971718773a11fd32cd7))
* reorder CLI command match arms for logical grouping ([741ad9b](https://github.com/z1-0/ah.sh/commit/741ad9b3daa1d1838a5c6c27cc5a4d23d2647c7a))
* reorder nix develop setup and add last_used_at touch ([34ac9d3](https://github.com/z1-0/ah.sh/commit/34ac9d3e13c98282774966848743955a6d74488c))
* replace ProviderInfo with ProviderType helpers ([d0cd5fc](https://github.com/z1-0/ah.sh/commit/d0cd5fcf12318a7b24eca5c78bab11a451f83f4c))
* restore completion subcommand for dynamic generation ([165f223](https://github.com/z1-0/ah.sh/commit/165f2233c678adb8a09e1cedd1b307d95116f179))
* restructure config initialization and path module organization ([282d764](https://github.com/z1-0/ah.sh/commit/282d7641f7a1f9bb3cbc7a558c9d4c88ab7f08b3))
* return Result&lt;Session&gt; from try_session_by_* ([b786c15](https://github.com/z1-0/ah.sh/commit/b786c15e3f7cd19cc2270941ed9829fbb3f70f6f))
* return WorkerGuard from log init instead of leaking it ([a1a739f](https://github.com/z1-0/ah.sh/commit/a1a739f5128127c4a8a3346fe8577810de2e2730))
* session/service.rs use anyhow + tracing ([7da311e](https://github.com/z1-0/ah.sh/commit/7da311eaf72e47ba86aa04f80b2ba3e14db02e58))
* session/types.rs remove SessionError, use anyhow ([0bf33b6](https://github.com/z1-0/ah.sh/commit/0bf33b6e106fe50d4a06ee0635f4a57286f9705a))
* **session:** centralize public types and update call sites ([043f51a](https://github.com/z1-0/ah.sh/commit/043f51a3f53c96478a4be93a71d962f72d84e999))
* **session:** consolidate session model and remove in-tree tests ([d7ff775](https://github.com/z1-0/ah.sh/commit/d7ff775440a7fa8aab6086c3b5644303f2f347ab))
* **session:** move session ordering to directory mtime ([f519066](https://github.com/z1-0/ah.sh/commit/f519066ec7707f23901f281600567b67ac7a0519))
* **session:** remove app layer wrapper ([1900d7c](https://github.com/z1-0/ah.sh/commit/1900d7cc88c25d93e201dfec2c1df1211879ce25))
* **session:** simplify unified session model usage ([c2d5efb](https://github.com/z1-0/ah.sh/commit/c2d5efb89fd5fed109a5f5d36aefe2c3b25aaa7e))
* show nix flake update output inline and simplify return type ([7c539a1](https://github.com/z1-0/ah.sh/commit/7c539a14c56e1c7fc222f16bdf90eab9eaf0597f))
* simplify check_nix_available with chain-style error handling ([5376112](https://github.com/z1-0/ah.sh/commit/53761128134570b8173e0693ba2da06e92cee514))
* simplify check_nix_available with match expression ([48b3de7](https://github.com/z1-0/ah.sh/commit/48b3de7df0441be1975c7547917052ba203d025f))
* simplify CLI args in types.rs and make provider optional ([fb8d204](https://github.com/z1-0/ah.sh/commit/fb8d204a5f938c3075910dfb737bc5c5f8b53557))
* simplify dev-templates provider and fix index mapping bug ([391baab](https://github.com/z1-0/ah.sh/commit/391baab1c098000e6018154914d23da12b6fb08e))
* simplify language grouping with bucket sort approach ([3414226](https://github.com/z1-0/ah.sh/commit/3414226474d0fad365da852457e1b10ab24b7b6d))
* simplify nix develop profile path handling ([013e4a8](https://github.com/z1-0/ah.sh/commit/013e4a8651f5fee07a27c8ed1d4c2d131a665d2d))
* simplify provider API and enhance provider information output ([1e410a9](https://github.com/z1-0/ah.sh/commit/1e410a9b60ce78b05841be906be389b09ef1563b))
* simplify provider/session dispatch and clarify nix develop intent ([ca66968](https://github.com/z1-0/ah.sh/commit/ca6696807441eac8e916327406fe114460b21055))
* simplify session execution and add debug logging ([0312730](https://github.com/z1-0/ah.sh/commit/0312730e01c8f493519d8e1546ba20022553e73f))
* simplify session history display with reusable table ([07abac0](https://github.com/z1-0/ah.sh/commit/07abac029b24e30f277f2827313066e4b4e81804))
* simplify session storage with helper functions and remove TOCTOU ([35e0aec](https://github.com/z1-0/ah.sh/commit/35e0aec2132ef5df47aa96d9b412956230416c23))
* split exec_nix_develop to use profile for builder and session ([716a9ce](https://github.com/z1-0/ah.sh/commit/716a9ce8842b58fe01a2643b1a614a1d6c1b4afa))
* storage.rs use anyhow ([e119c0c](https://github.com/z1-0/ah.sh/commit/e119c0c56295004cd138798defd78f378245941c))
* switch provider abstraction to function dispatch ([553d69b](https://github.com/z1-0/ah.sh/commit/553d69be860564673344f037b713e4f5b51fa2f2))
* unify anyhow usage patterns ([f62a594](https://github.com/z1-0/ah.sh/commit/f62a594f3a1f53de014fe3413940780f424e9ac1))
* unify import style across codebase ([ab7c12f](https://github.com/z1-0/ah.sh/commit/ab7c12f8b8bf46f7aba8576ef45460821275a055))
* unify provider selection flow ([a8b5831](https://github.com/z1-0/ah.sh/commit/a8b58313753502fda8ff5d3b36a331a7cfc9ba42))
* use &&let syntax and remove unnecessary clone ([c11d0b2](https://github.com/z1-0/ah.sh/commit/c11d0b29f000f16e3706ba42943d0ebb8a07c396))
* use anyhow::Result and fully qualified macro paths ([2fe49e4](https://github.com/z1-0/ah.sh/commit/2fe49e4a272ae1b9a3f574ae6f181a36833f5499))
* use APP_NAME constant and handle missing session dir ([5043cb9](https://github.com/z1-0/ah.sh/commit/5043cb995935b061bde60c5dfb53396675da2abe))
* use bail! instead of anyhow! for error returns ([8d9e5d4](https://github.com/z1-0/ah.sh/commit/8d9e5d4ca94f4d2b564908b701617544e6092f75))
* use DateTime&lt;Utc&gt; for timestamp instead of String ([bfedfd3](https://github.com/z1-0/ah.sh/commit/bfedfd395d822b122bfb46e7ae0ddc0f1c40fcc0))

## [Unreleased]

## [0.1.1](https://github.com/z1-0/ah.sh/compare/v0.1.0...v0.1.1) - 2026-07-04

### Added

- *(build)* replace fenix with rust-overlay for toolchain

## [0.1.0](https://github.com/z1-0/ah.sh/releases/tag/v0.1.0) - 2026-07-03

### Added

- move schemars to dev-dependencies with cfg_attr(test)
- modify provider table display
- modify session table display
- improve session list time display with human-readable relative format
- add language help text to session completion candidates
- add dynamic shell completions for languages and session keys
- switch to dynamic shell completion
- add clap_complete dependency
- add last_used_at and last_updated_at to session metadata
- prevent data corruption via atomic file writes
- log nix version on check_nix_available success
- add structured logging with tracing and #[instrument]
- add logging infrastructure with tracing subscriber
- add custom shell detection with libc integration
- add Nix availability check before command dispatch
- add user configuration support with module refactoring
- preload config on startup to auto-create default
- add AppConfig struct and load_config()
- add config module with get_config_path()
- trigger session history from restore command instead of use
- add session history prompt in CLI
- record directory history when entering session
- add print_session_history function
- export find_by_path and update_history in session service
- add update_history and find_by_path functions
- add HistoryEntry type for session history
- add cachix workflow for binary caching
- add top-level update and restore commands as session aliases
- add ah init command
- add ah update command
- add output beautification with table formatting
- add Init and Update command placeholders
- _(provider)_ parse dev-templates attrs from nix store with cache-first flow
- _(provider)_ add attrs cache with corruption-safe fallback
- _(provider)_ add injectable store resolver with prefetch fallback
- lock dev-templates flake inputs
- _(cli)_ rename lang command to use
- _(cli)_ rename create to lang command
- _(cli)_ add provider subcommand suite
- _(dx)_ colorize warnings on tty
- _(cli)_ print help on no args and exit 2
- _(cli)_ print help on no args and exit 2
- add session clear/remove with typed selector and fixed id length
- cap concurrent dev-template fetch workers
- deduplicate languages before session generation
- bind generated flake.nix to session-specific cache directories
- implement dynamic Nix shell template generation with AST parsing
- implement dynamic Nix shell template generation with AST parsing for dev-templates
- implement session management and profile support
- change default provider to dev-templates
- remove formatter/linter provisioning and update AGENTS.md build priority
- add language normalization layer and cli provider support
- implement static language lists and weekly CI update
- decouple provider logic and add multi-provider support

### Fixed

- _(ci)_ exclude CHANGELOG.md from treefmt to prevent release-plz format conflicts
- set initial version to 0.1.0 to enable release-plz release
- use git-only mode in release-plz for non-published package
- configure release-plz to handle non-publishable package
- guard default config write with existence check
- _(ci)_ modify release-plz token
- _(session)_ render inner value in SessionKey Display
- translate check_nix_available error message to English
- restore lost tracing-attributes dependency and sync Cargo.lock
- ensure logs are flushed before process replacement and on exit
- correct print_error call with single formatted string argument
- update is_terminal call to is_interactive in manager.rs
- restore info message when no session history found
- correct include_str path and remove JsonSchema dependency
- determine --profile usage by checking if profile file exists
- optimize session operations and remove redundant path computation
- clear current session when removing or clearing sessions
- update comment and optimize string allocation in storage
- use HISTORY_LIMIT constant instead of hardcoded 5
- use binary name for app name and align crate name
- auto-enter dev shell after flake update instead of prompting
- properly handle nix flake update errors
- session/mod.rs remove unresolved SessionError import
- _(executor)_ correct nix develop argument flow for session lifecycle
- _(paths)_ use ProjectDirs data/cache roots directly
- _(provider)_ use valid nix flake prefetch fallback command
- _(provider)_ degrade cache read errors to warning and miss
- persist session metadata when reusing flake
- reuse existing session flake without overwrite

### Other

- release v0.0.0 ([#22](https://github.com/z1-0/ah.sh/pull/22))
- _(cargo)_ simplify release profile
- use GH_PAT for release-plz to trigger downstream workflows
- standardize job and step naming in workflows
- drop explicit permissions block
- _(cachix)_ simplify workflow matrix and nix build
- _(dependabot)_ group nix and github-actions updates
- limit pull_request trigger and add paths-ignore
- _(nix)_ bump nixpkgs from `7a1a647` to `e52c192` ([#20](https://github.com/z1-0/ah.sh/pull/20))
- bump actions/checkout from 6 to 7 ([#16](https://github.com/z1-0/ah.sh/pull/16))
- _(nix)_ bump fenix from `df161b9` to `16810aa` ([#17](https://github.com/z1-0/ah.sh/pull/17))
- _(nix)_ bump git-hooks-nix from `3bbec39` to `9f7e991` ([#18](https://github.com/z1-0/ah.sh/pull/18))
- _(nix)_ bump advisory-db from `1e3b508` to `4075127` ([#21](https://github.com/z1-0/ah.sh/pull/21))
- _(cargo)_ bump the cargo-minor-and-patch group with 3 updates ([#19](https://github.com/z1-0/ah.sh/pull/19))
- _(dependabot)_ fix invalid scope and versioning-strategy
- add aarch64-linux to Cachix build matrix
- add nix flake check workflows
- add release-plz
- _(nix)_ migrate to fenix + crane
- bump actions/checkout from 6 to 7 ([#15](https://github.com/z1-0/ah.sh/pull/15))
- bump config from 0.15.23 to 0.15.24 in the cargo-minor-and-patch group ([#14](https://github.com/z1-0/ah.sh/pull/14))
- update supported languages list ([#13](https://github.com/z1-0/ah.sh/pull/13))
- add unit tests for cli and provider
- add unit tests for privider and session
- migrate flake updates to dependabot
- bump cachix/cachix-action from 15 to 17 ([#12](https://github.com/z1-0/ah.sh/pull/12))
- update dependabot.yaml
- _(deps)_ bump the cargo-minor-and-patch group with 3 updates ([#11](https://github.com/z1-0/ah.sh/pull/11))
- _(deps)_ bump peter-evans/create-pull-request from 6 to 8 ([#10](https://github.com/z1-0/ah.sh/pull/10))
- _(deps)_ bump actions/checkout from 4 to 6 ([#8](https://github.com/z1-0/ah.sh/pull/8))
- _(deps)_ bump DeterminateSystems/update-flake-lock from 24 to 28 ([#9](https://github.com/z1-0/ah.sh/pull/9))
- _(deps)_ bump cachix/install-nix-action from 30 to 31 ([#7](https://github.com/z1-0/ah.sh/pull/7))
- _(deps)_ bump actions/github-script from 7 to 9 ([#6](https://github.com/z1-0/ah.sh/pull/6))
- add dependabot and flake.lock update workflow
- update supported languages list ([#5](https://github.com/z1-0/ah.sh/pull/5))
- update supported languages list ([#4](https://github.com/z1-0/ah.sh/pull/4))
- add readme
- update flake.lock
- show nix flake update output inline and simplify return type
- deduplicate parent-dir creation and simplify error handling
- propagate config errors instead of inline process::exit
- integrate LogLevel enum into log filtering with release profile
- extract LogLevel enum and integrate into config
- restore completion subcommand for dynamic generation
- reorder nix develop setup and add last_used_at touch
- unify import style across codebase
- clean up fs-err usage, fix TOCTOU and deduplicate history logic
- complete std::fs to fs-err migration and clean context
- migrate to fs-err and clean context
- migrate to fs-err and clean context
- add fs-err dependency
- return Result<Session> from try*session_by*\*
- optimize lookup logic and improve error context
- refine error messages and log output
- add #[instrument] tracing to cmd, session and dev_templates modules
- add structured logging to cmd, session and dev_templates modules
- add #[instrument] to manager functions and clean up output
- update flake.lock
- backup and clean up release-please workflow
- migrate to release-please with Cargo.lock fixup support
- rename stderr_layer to console_layer
- simplify check_nix_available with match expression
- clean up logging implementation issues
- return WorkerGuard from log init instead of leaking it
- unify anyhow usage patterns
- remove redundant check_nix_available calls
- use anyhow::Result and fully qualified macro paths
- simplify check_nix_available with chain-style error handling
- inline build_nix_develop_cmd into nix_develop_of_session
- format code with cargo fmt
- migrate from console to crossterm
- remove implicit use command and simplify CLI structure
- clean up utils.rs and optimize buffer size
- simplify CLI args in types.rs and make provider optional
- inline exec/run in cmd.rs and simplify error messages
- simplify language grouping with bucket sort approach
- update flake.lock
- upgrade clap to 4.6.1
- exclude config.schema.json from treefmt and restructure config
- upgrade schemars to 1.2 and toml to 1.1, add $schema to default config
- simplify provider API and enhance provider information output
- remove redundant error context messages and comments
- improve build error messages with specific context
- restructure config initialization and path module organization
- update dependencies and improve error messages
- HashMap capacity pre-allocation and clean up provider list
- code cleanup and formatting improvements
- migrate OnceLock to LazyLock and simplify provider initialization
- remove redundant comments
- rename to path.rs and optimize module structure
- translate config-related comments to English
- remove ProviderShowSelector, use ProviderType directly
- remove unused use_profile parameter from nix_develop_of_session
- rename path constants for clarity
- reorder CLI command match arms for logical grouping
- update supported languages list ([#3](https://github.com/z1-0/ah.sh/pull/3))
- simplify nix develop profile path handling
- centralize session file/directory names in paths.rs
- increase HISTORY_LIMIT from 3 to 64
- inline read_session_from_path and update return types
- simplify session storage with helper functions and remove TOCTOU
- extract session reading logic and use FP style
- rename session lookup functions for consistency
- extract session lookup functions to storage module
- extract session lookup helpers and simplify remove_sessions
- optimize session lookups and remove unused find_in_list
- finalize session module structure
- move non-storage logic to mod.rs and improve naming
- remove service layer in session module
- improve error handling and simplify current session cleanup
- remove chrono dependency and simplify history.json
- simplify session history display with reusable table
- combine nested if-let conditions with && operator
- use &&let syntax and remove unnecessary clone
- use DateTime<Utc> for timestamp instead of String
- add get_cwd utility and deduplicate current_dir calls
- extract HISTORY_LIMIT constant for session history
- update Cargo.lock with chrono dependency
- add .worktrees to gitignore
- use APP_NAME constant and handle missing session dir
- remove unused code and add provider alias cache
- remove init command and fix crate name
- remove documentation files
- migrate to release-plz for automated releases
- add enterprise-ready project setup
- extract provider types to dedicated types.rs module
- extract shared nix develop command setup
- improve CLI help text for better readability
- remove duplicate exec function, reuse from cmd module
- introduce domain-specific type aliases and consolidate language handling
- extract provider detection logic in implicit use
- move Provider loading logic to provider module
- consolidate provider module and rename types
- update supported languages list ([#2](https://github.com/z1-0/ah.sh/pull/2))
- rename language update workflow and outputs
- unify provider selection flow
- language module and centralize language normalization
- remove hardcoded exit-code string matching
- consolidate language mapping functions and remove duplication
- improve error handling in get_nix_store_path
- simplify dev-templates provider and fix index mapping bug
- remove tracing dependency and use print/eprintln for CLI output
- cleanup language_maps.rs
- use bail! instead of anyhow! for error returns
- optimize language normalization and session lookup
- eliminate boolean parameter smell and strengthen type safety
- simplify provider/session dispatch and clarify nix develop intent
- remove session_dir field and simplify control flow
- switch provider abstraction to function dispatch
- replace ProviderInfo with ProviderType helpers
- remove EnsureFilesResult from provider flow
- provider/dev_templates/mod.rs use anyhow + tracing
- provider/language_maps.rs use anyhow
- provider/devenv/mod.rs use anyhow
- provider/types.rs remove AppWarning, use anyhow
- provider/registry.rs use anyhow
- manager.rs use tracing for logs, keep println for output
- session/service.rs use anyhow + tracing
- cmd.rs use anyhow, replace eprintln with tracing
- session/types.rs remove SessionError, use anyhow
- storage.rs use anyhow
- cli/mod.rs use anyhow
- paths.rs use anyhow
- delete error.rs and warning.rs - using anyhow + tracing instead
- remove error/warning module exports from lib.rs
- add tracing init in main.rs
- add anyhow/tracing, remove thiserror
- add anyhow tracing refactor implementation plan
- update error logging tracing design - all anyhow + tracing
- add error logging tracing design spec
- _(dev_templates)_ inline store_resolver functions and simplify prefetch
- _(dev_templates)_ single prefetch for dev-templates repo
- _(dev_templates)_ replace manual thread pool with rayon
- _(dev_templates)_ simplify store_resolver by removing over-engineered abstraction
- _(cmd)_ centralize command execution and errors
- _(cmd)_ rename executor module
- _(provider)_ streamline dev-templates prefetch parsing
- _(debug)_ simplify exec command logging output
- _(session)_ simplify unified session model usage
- _(session)_ centralize public types and update call sites
- _(session)_ move session ordering to directory mtime
- _(session)_ consolidate session model and remove in-tree tests
- _(provider)_ centralize attrs cache path and remove dev-templates tests
- _(provider)_ remove legacy dev-templates fetcher path
- _(tests)_ reduce resolver fake runner type complexity
- _(provider)_ keep task5 scoped to flake generator
- _(provider)_ switch dev-templates pipeline to lock-probe + store resolver
- _(provider)_ stabilize resolver tests with injectable flake reader
- _(provider)_ harden resolver error mapping and flake reading
- _(provider)_ format gen file
- _(paths)_ move session dir helper to paths module
- refresh flake inputs and path helpers
- _(provider)_ remove unused registry helpers
- _(provider)_ separate metadata from shell execution
- remove test code and dependencies
- rename providers module to provider
- move --provider to use subcommand and implement robust implicit use induction
- update README with installation guides and use cases
- add cachix github action workflow
- _(providers)_ consolidate language map handling
- _(providers)_ move language mappings per provider
- _(paths)_ use directories crate for xdg paths
- _(session)_ remove app layer wrapper
- update README usage guide
- clear
- _(format)_ drop markdownlint
- _(deps)_ bump clap/rnix/rowan
- _(dx)_ avoid cloning warnings when printing
- _(fmt)_ apply rustfmt after warning order test
- _(dx)_ ensure warning order is stable
- _(fmt)_ apply rustfmt after adding tests
- _(providers)_ strengthen nix parser and flake generator tests
- _(providers)_ cover flake parsing and generation
- _(providers)_ cover language normalize and validate
- _(session)_ cover SessionKey parsing
- _(fmt)_ format SessionApp imports
- _(app)_ route session create through SessionApp
- _(app)_ route session restore through SessionApp
- _(app)_ route session remove/clear through SessionApp
- _(app)_ restrict SessionApp visibility
- _(app)_ route session list through SessionApp
- add trailing newlines to layer modules
- add layering module skeleton
- _(providers)_ make language normalization fallible
- _(providers)_ fail-fast on invalid language aliases
- plumb structured warnings and remove library eprintln
- _(cli)_ avoid unreachable when session restore diverges
- _(executor)_ return diverging Result and surface exec errors
- update Cargo.lock
- rustfmt session module
- Revert "feat(cli): print help on no args and exit 2"
- reduce brittleness in CLI contract assertions
- tighten CLI no-args help contract
- add CLI contract
- remove stray completion promise from plan
- add project-wide refactor implementation plan
- tighten refactor spec contracts
- add project-wide refactor design spec
- align session module exports
- move session service
- add session storage
- add session struct
- add session module types
- add session module types
- add session module refactor plan
- ignore .worktrees directory
- clarify session module refactor spec
- add session module refactor design
- comment on no-change runs
- update supported languages list ([#1](https://github.com/z1-0/ah.sh/pull/1))
- grant workflow write permissions for automated PRs
- refactor session management and remove flow
- rename selector var
- move SessionError into sessions module
- rename AhError to AppError
- cover best-effort cache write fallback
- reduce session setup overhead and lint noise
- optimize project structure and improve separation of concerns
- simplify session execution and add debug logging
- remove ProviderAssetManager and simplify provider architecture
- split exec_nix_develop to use profile for builder and session
- initialize editorconfig and update nix flakes
- implement senior-architect-grade refactoring (custom errors, DRY providers, and decoupled CLI)
- embed provider templates as assets and sync to ~/.local/share/ah at runtime
- abstract provider logic into traits and optimize resource path handling
