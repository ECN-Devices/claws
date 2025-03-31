{
  lib,
  makeRustPlatform,
  rust-bin,
  pkg-config,
  systemd,
  libGL,
  libxkbcommon,
  udev,
  vulkan-loader,
  patchelf,
  wayland,
}: let
  manifest = (lib.importTOML ./Cargo.toml).package;
  rustPlatform = makeRustPlatform {
    cargo = rust-bin.stable.latest.default;
    rustc = rust-bin.stable.latest.default;
  };
in
  rustPlatform.buildRustPackage {
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ./Cargo.lock;
    src = lib.cleanSource ./.;

    doCheck = false;

    nativeBuildInputs = [
      pkg-config
      systemd
      patchelf
    ];

    buildInputs = [
      libGL
      libxkbcommon
      udev
      vulkan-loader
      wayland
    ];

    fixupPhase = ''
      mkdir -p $out/lib

      cp ${udev}/lib/libudev.so* $out/lib/
      cp ${wayland}/lib/libwayland* $out/lib/
      cp ${libxkbcommon}/lib/libxkbcommon.so* $out/lib/
      cp ${libGL}/lib/* $out/lib/
      cp ${vulkan-loader}/lib/* $out/lib/
      patchelf --set-rpath $out/lib $out/bin/claws
    '';
  }
