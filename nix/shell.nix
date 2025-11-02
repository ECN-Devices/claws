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

  moldDevShell = craneLib.devShell.override {
    # For example, use the mold linker
    mkShell = pkgs.mkShell.override {
      stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.stdenv;
    };
  };
in
  moldDevShell {
    inherit (self.checks.${pkgs.system}.git-hooks) shellHook;

    inputsFrom = with pkgs; [
      self.packages.${system}.default
    ];

    packages = with pkgs; [
      self.checks.${system}.git-hooks.enabledPackages
      cachix
      cargo-nextest
    ];

    LD_LIBRARY_PATH = libPath;
  }
