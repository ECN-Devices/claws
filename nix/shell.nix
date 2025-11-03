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
    mkShell = pkgs.mkShell.override {
      stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.stdenv;
    };
  };
in
  moldDevShell {
    inputsFrom = with pkgs; [
      self.packages.${system}.default
    ];

    packages = with pkgs; [
      cachix
      cargo-nextest
    ];

    LD_LIBRARY_PATH = libPath;
  }
