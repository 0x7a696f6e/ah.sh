{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    git-hooks-nix = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    systems = {
      url = "github:nix-systems/default";
      flake = false;
    };
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs =
    { self, ... }@inputs:

    with inputs;

    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import systems;

      imports = [
        treefmt-nix.flakeModule
        git-hooks-nix.flakeModule
      ];

      perSystem =
        {
          config,
          system,
          lib,
          ...
        }:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };

          craneLib = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.stable.latest.default);

          src = lib.cleanSourceWith {
            filter =
              path: type: (builtins.match ".*assets/.*" path) != null || craneLib.filterCargoSources path type;
            src = ./.;
          };

          commonArgs = {
            inherit src;
            strictDeps = true;
          };

          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          ah = craneLib.buildPackage (
            commonArgs
            // {
              inherit cargoArtifacts;
              doCheck = false;
            }
          );
        in
        {
          checks = {
            inherit ah;
            cargoClippy = craneLib.cargoClippy (
              commonArgs
              // {
                inherit cargoArtifacts;
                cargoClippyExtraArgs = "--all-targets -- --deny warnings";
              }
            );
            cargoAudit = craneLib.cargoAudit { inherit src advisory-db; };
            cargoNextest = craneLib.cargoNextest (commonArgs // { inherit cargoArtifacts; });
          };

          packages = {
            inherit ah;
            default = ah;
          };

          apps.default = {
            type = "app";
            program = lib.getExe' ah "ah";
          };

          devShells.default = craneLib.devShell {
            inherit (config.pre-commit.devShell) shellHook;
            checks = self.checks.${system};
            packages =
              (lib.attrValues config.treefmt.build.programs)
              ++ config.pre-commit.settings.enabledPackages
              ++ [
                pkgs.rust-bin.stable.latest.rust-analyzer
                pkgs.rust-bin.stable.latest.rust-src
              ];
          };

          pre-commit.settings.hooks = {
            treefmt = {
              enable = true;
              package = config.treefmt.build.wrapper;
            };
          }
          //
            lib.genAttrs
              [
                "check-json"
                "check-toml"
                "check-xml"
                "check-yaml"
                "editorconfig-checker"
                "fix-byte-order-marker"
                "flake-checker"
                "mixed-line-endings"
                "trim-trailing-whitespace"
              ]
              (_: {
                enable = true;
              });

          treefmt.programs = {
            nixfmt.enable = true;
            prettier.enable = true;
            taplo.enable = true;
            rustfmt = {
              enable = true;
              package = pkgs.rust-bin.stable.latest.rustfmt;
            };
          };
          treefmt.settings.excludes = [
            "src/assets/config.schema.json"
            "CHANGELOG.md"
          ];
        };
    };
}
