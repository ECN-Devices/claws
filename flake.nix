{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    pre-commit-hooks = {
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
      in {
        packages = {
          default = pkgs.callPackage ./nix/default.nix {inherit pkgs;};
        };

        formatter = pkgs.alejandra;

        checks.pre-commit-check = inputs.pre-commit-hooks.lib.${system}.run {
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

            check-yaml.enable = true;
            yamlfmt = {
              enable = true;
              package = pkgs.yaml-language-server;
            };

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

        devShells = {
          default = pkgs.callPackage ./nix/shell.nix {};
          git = pkgs.mkShell {
            inherit (self.checks.${system}.pre-commit-check) shellHook;
            buildInputs = self.checks.${system}.pre-commit-check.enabledPackages;
          };
          # git_test = pre-commit-lib.mkDevShell {
          #   extraPackages = with pkgs; [
          #     alejandra
          #     taplo
          #     cargo-nextest
          #     pkg-config
          #     systemd
          #     (rust-bin.stable.latest.default.override {
          #       extensions = [
          #         "cargo"
          #       ];
          #     })
          #   ];
          # };
        };
      }
    );
}
