{
  self,
  pkgs,
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
      cargo-xwin
      cargo-nextest
    ];

    LD_LIBRARY_PATH = libPath;
  }
