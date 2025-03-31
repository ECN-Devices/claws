{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    pre-commit-env = {
      url = "github:chenow/nix-pre-commit";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    pre-commit-env,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        pre-commit-lib = pre-commit-env.lib.${system};
      in {
        packages = {
          default = pkgs.callPackage ./default.nix {};
          gitBuild = pkgs.callPackage ./gitBuild.nix {};
        };

        devShells = {
          default = pkgs.callPackage ./shell.nix {};
          git = pre-commit-lib.mkDevShell {
            extraPackages = with pkgs; [
              alejandra
              taplo
              cargo-nextest
              pkg-config
              systemd
              (rust-bin.stable.latest.default.override {
                extensions = [
                  "cargo"
                ];
              })
            ];
          };
        };
      }
    );
}
