{pkgs, ...}: let
  libPath = with pkgs;
    lib.makeLibraryPath [
      libGL
      libxkbcommon
      wayland
    ];
in
  pkgs.mkShell {
    inputsFrom = [(pkgs.callPackage ./default.nix {})];
    nativeBuildInputs = with pkgs; [
      pkg-config
      systemd
      cargo-xwin
      cargo-nextest
      (rust-bin.stable.latest.default.override {
        extensions = [
          "rust-src"
          "rustc"
          "cargo"
          "rust-analyzer"
        ];
        targets = [
          "x86_64-unknown-linux-gnu"
          "x86_64-pc-windows-msvc"
        ];
      })
    ];

    buildInputs = with pkgs; [
    ];

    LD_LIBRARY_PATH = libPath;
  }
