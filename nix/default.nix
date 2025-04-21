{
  pkgs,
  rust-bin,
  ...
}: let
  manifest = (pkgs.lib.importTOML ../Cargo.toml).package;
  rustPlatform = pkgs.makeRustPlatform {
    cargo = rust-bin.stable.latest.default;
    rustc = rust-bin.stable.latest.default;
  };
in
  rustPlatform.buildRustPackage (finalAttrs: {
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ../Cargo.lock;
    src = pkgs.lib.cleanSource ../.;

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

    libPath = with pkgs;
      lib.makeLibraryPath [
        libGL
        libxkbcommon
        udev
        vulkan-loader
        wayland
      ];

    fixupPhase = ''
      patchelf --set-rpath $libPath $out/bin/${finalAttrs.pname}
    '';
  })
