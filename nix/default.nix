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
    src = craneLib.path ../.;
    strictDeps = true;
    buildType = "debug";
    doCheck = false;

    nativeBuildInputs = with pkgs; [
      pkg-config
    ];

    buildInputs = with pkgs; [
      systemd
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
