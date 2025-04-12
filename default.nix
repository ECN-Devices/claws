{
  pkgs,
  rust-bin,
  ...
}: let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
  rustPlatform = pkgs.makeRustPlatform {
    cargo = rust-bin.stable.latest.default;
    rustc = rust-bin.stable.latest.default;
  };
in
  rustPlatform.buildRustPackage (finalAttrs: {
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;

    doCheck = false;

    nativeBuildInputs = with pkgs; [
      pkg-config
      systemd
      patchelf
    ];

    buildInputs = with pkgs; [
      libGL
      libxkbcommon
      udev
      vulkan-loader
      wayland
    ];

    fixupPhase = with pkgs; ''
      mkdir -p $out/lib

      cp ${udev}/lib/libudev.so* $out/lib/
      cp ${wayland}/lib/libwayland* $out/lib/
      cp ${libxkbcommon}/lib/libxkbcommon.so* $out/lib/
      cp ${libGL}/lib/* $out/lib/
      cp ${vulkan-loader}/lib/* $out/lib/
      patchelf --set-rpath $out/lib $out/bin/${finalAttrs.pname}
    '';
  })
