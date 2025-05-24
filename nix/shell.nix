{
  self,
  pkgs,
  rust-bin,
  craneLib,
  ...
}: let
  libPath = with pkgs;
    lib.makeLibraryPath [
      libGL
      libxkbcommon
      wayland
    ];
in
  craneLib.devShell {
    inherit (self.checks.${pkgs.system}.git-hooks) shellHook;

    inputsFrom = with pkgs; [
      self.packages.${system}.default
    ];

    packages = with pkgs; [
      self.checks.${system}.git-hooks.enabledPackages

      pkg-config
      systemd
      cargo-xwin
      cargo-nextest
      (rust-bin.stable.latest.default.override {
        extensions = [
          "rust-analyzer"
        ];
        targets = [
          "x86_64-unknown-linux-gnu"
          "x86_64-pc-windows-msvc"
        ];
      })
    ];

    LD_LIBRARY_PATH = libPath;
  }
