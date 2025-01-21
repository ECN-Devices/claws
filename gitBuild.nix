{
  lib,
  makeRustPlatform,
  rust-bin,
  fetchFromGitLab,
  pkg-config,
  libxkbcommon,
  udev,
  vulkan-loader,
  stdenv,
  darwin,
  wayland,
  libGL,
  patchelf,
}: let
  rustPlatform = makeRustPlatform {
    cargo = rust-bin.stable.latest.default;
    rustc = rust-bin.stable.latest.default;
  };
  repoBranch = "dev";
in
  rustPlatform.buildRustPackage rec {
    pname = "claws";
    version = "0.1.0";

    src = fetchFromGitLab {
      owner = "lapa-ecn";
      repo = "claws";
      rev = "refs/heads/${repoBranch}";
      hash = lib.fakeHash;
    };

    cargoHash = lib.fakeHash;

    buildType = "debug";

    doCheck = false;

    nativeBuildInputs = [
      pkg-config
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

    meta = {
      description = "";
      homepage = "https://gitlab.com/lapa-ecn/claws";
      license = lib.licenses.agpl3Plus;
      maintainers = with lib.maintainers; [];
      mainProgram = "claws";
    };
  }
