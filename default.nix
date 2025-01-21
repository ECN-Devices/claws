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
  darwin,
  stdenv,
  patchelf,
  wayland,
}: let
  manifest = (lib.importTOML ./Cargo.toml).package;
  rustPlatform = makeRustPlatform {
    cargo = rust-bin.stable.latest.default;
    rustc = rust-bin.stable.latest.default;
  };
in
  rustPlatform.buildRustPackage rec {
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

    buildInputs =
      [
        libGL
        libxkbcommon
        udev
        vulkan-loader
      ]
      ++ lib.optionals stdenv.isDarwin [
        darwin.apple_sdk.frameworks.AppKit
        darwin.apple_sdk.frameworks.CoreFoundation
        darwin.apple_sdk.frameworks.CoreGraphics
        darwin.apple_sdk.frameworks.IOKit
        darwin.apple_sdk.frameworks.Metal
        darwin.apple_sdk.frameworks.QuartzCore
        darwin.apple_sdk.frameworks.SystemConfiguration
      ]
      ++ lib.optionals stdenv.isLinux [
        wayland
      ];

    installPhase = ''
      mkdir -p $out/bin
      mkdir -p $out/lib
      cp target/x86_64-unknown-linux-gnu/debug/claws $out/bin/

      cp ${udev}/lib/libudev.so* $out/lib/
      cp ${wayland}/lib/libwayland* $out/lib/
      cp ${libxkbcommon}/lib/libxkbcommon.so* $out/lib/
      cp ${libGL}/lib/* $out/lib/
      cp ${vulkan-loader}/lib/* $out/lib/
      patchelf --set-rpath $out/lib $out/bin/claws
    '';
  }
