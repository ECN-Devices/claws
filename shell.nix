{pkgs ? import <nixpkgs> {}}: let
  libPath = with pkgs;
    lib.makeLibraryPath [
      libGL
      libxkbcommon
      wayland
    ];
in
  pkgs.mkShell {
    # Get dependencies from the main package
    inputsFrom = [(pkgs.callPackage ./default.nix {})];
    # Additional tooling
    nativeBuildInputs = with pkgs; [
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

    buildInputs = with pkgs; [
    ];

    LD_LIBRARY_PATH = libPath;
  }
