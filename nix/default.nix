{
  pkgs,
  craneLib,
  ...
}: let
  libPath = with pkgs;
    lib.makeLibraryPath [
      libGL
      libxkbcommon
      udev
      vulkan-loader
      wayland
    ];
in
  craneLib.buildPackage {
    src = ../.;
    strictDeps = true;

    doCheck = false;

    nativeBuildInputs = with pkgs; [
      pkg-config
      systemd
    ];

    buildInputs = with pkgs; [
      libGL
      libxkbcommon
      udev
      vulkan-loader
      wayland
    ];

    LD_LIBRARY_PATH = libPath;

    postInstall = ''
      patchelf --set-rpath ${libPath} $out/bin/claws
    '';
  }
