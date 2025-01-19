{pkgs ? import <nixpkgs> {}}: let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  pkgs.rustPlatform.buildRustPackage rec {
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;

    buildInputs = with pkgs; [
      pkg-config
      systemd
      (rust-bin.stable.latest.default.override {
        extensions = [
          "rust-src"
          "rustc"
          "cargo"
          "rust-analyzer"
        ];
      })
    ];

    nativeBuildInputs = with pkgs; [
      pkg-config
      systemd
    ];

    doCheck = false;
  }
