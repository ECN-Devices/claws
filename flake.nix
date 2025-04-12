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
    ...
  } @ inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import inputs.rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        pre-commit-lib = inputs.pre-commit-env.lib.${system};
      in {
        packages = {
          default = pkgs.callPackage ./default.nix {};
        };

        formatter = pkgs.alejandra;

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
