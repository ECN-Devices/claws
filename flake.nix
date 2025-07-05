{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    crane.url = "github:ipetkov/crane";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    ...
  } @ inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import inputs.rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain (
          pkgs.rust-bin.stable.latest.default.override {
            extensions = [
              "rust-analyzer"
              "cargo"
              "rustc"
              "rust-src"
            ];
            targets = [
              "x86_64-unknown-linux-gnu"
            ];
          }
        );
      in {
        checks = {
          build = self.packages.${system}.default;

          git-hooks = inputs.git-hooks.lib.${system}.run {
            src = ./.;
            enabledPackages = with pkgs; [
              pkg-config
              systemd
            ];
            hooks = {
              alejandra = {
                enable = true;
                package = pkgs.alejandra;
                args = ["-q"];
              };
              deadnix = {
                enable = true;
                args = ["-e" "-q"];
              };
              statix = {
                enable = true;
                args = ["fix"];
              };

              check-json.enable = true;
              pretty-format-json.enable = true;

              check-toml.enable = true;
              taplo = {
                enable = true;
                package = pkgs.taplo;
              };

              cargo-check.enable = true;
              rustfmt = {
                enable = true;
                packageOverrides = {
                  cargo = pkgs.rust-bin.stable.latest.default;
                  rustfmt = pkgs.rust-bin.stable.latest.default;
                };
              };
              clippy = {
                enable = true;
                settings.allFeatures = true;
                packageOverrides = {
                  cargo = pkgs.rust-bin.stable.latest.default;
                  clippy = pkgs.rust-bin.stable.latest.default;
                };
              };

              trim-trailing-whitespace.enable = true;
              check-merge-conflicts.enable = true;
            };
          };
        };

        packages = {
          default = pkgs.callPackage ./nix/default.nix {inherit inputs craneLib;};
        };

        formatter = pkgs.alejandra;

        devShells = {
          default = pkgs.callPackage ./nix/shell.nix {inherit self inputs craneLib;};
          # git = pkgs.mkShell {
          #   inherit (self.checks.${system}.pre-commit-check) shellHook;
          #   buildInputs = self.checks.${system}.pre-commit-check.enabledPackages;
          # };
        };
      }
    );
}
