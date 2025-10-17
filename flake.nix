{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";

    flake-utils.url = "github:numtide/flake-utils";

    crane.url = "github:ipetkov/crane";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-substituters = ["https://claws.cachix.org"];
    extra-trusted-public-keys = ["claws.cachix.org-1:3gHGIPQan8LLHHuv71PFhdm438BZVIUfMjqvQZ22AIs="];
  };

  outputs = {
    self,
    nixpkgs,
    ...
  } @ inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import inputs.rust-overlay)];

        pkgs = import nixpkgs {
          inherit system overlays;
        };

        pkgsCross = import nixpkgs {
          inherit system overlays;
          crossSystem = {
            config = "x86_64-w64-mingw32";
            libc = "msvcrt";
          };
        };

        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain (
          p:
            p.rust-bin.stable.latest.default.override {
              extensions = [
                "rust-analyzer"
                "cargo"
                "rustc"
                "rust-src"
              ];
              targets = [
                "x86_64-unknown-linux-gnu"
              ];
            }
        );

        craneLibCross = (inputs.crane.mkLib pkgsCross).overrideToolchain (
          p:
            p.rust-bin.stable.latest.default.override {
              targets = [
                "x86_64-pc-windows-gnu"
              ];
            }
        );

        libPath = with pkgs;
          lib.makeLibraryPath [
            libGL
            libxkbcommon
            wayland
          ];

        commonArgs = {debug ? false}: {
          src = craneLib.path ./.;
          strictDeps = true;

          CARGO_PROFILE =
            if debug
            then "dev"
            else "release";

          doCheck =
            if debug
            then false
            else true;

          nativeBuildInputs = with pkgs; [
            pkg-config
            makeWrapper
          ];

          buildInputs = with pkgs; [
            systemd
          ];
        };

        windowsArgs = {debug ? false}: {
          src = craneLibCross.path ./.;
          strictDeps = true;

          CARGO_PROFILE =
            if debug
            then "dev"
            else "release";

          doCheck =
            if debug
            then false
            else true;
        };

        cargoArtifactsDebug = craneLib.buildDepsOnly (commonArgs {debug = true;});
        cargoArtifactsRelease = craneLib.buildDepsOnly (commonArgs {debug = false;});

        cargoClippyDebug = craneLib.buildDepsOnly (commonArgs {debug = true;}
          // {
            cargoArtifacts = cargoArtifactsDebug;
          });
        cargoClippyRelease = craneLib.buildDepsOnly (commonArgs {debug = false;}
          // {
            cargoArtifacts = cargoArtifactsRelease;
          });

        cargoArtifactsWindowsDebug = craneLibCross.buildDepsOnly (windowsArgs {debug = true;});
        cargoArtifactsWindowsRelease = craneLibCross.buildDepsOnly (windowsArgs {debug = false;});

        cargoClippyWindowsDebug = craneLibCross.cargoClippy (commonArgs {debug = true;}
          // {
            cargoArtifacts = cargoArtifactsWindowsDebug;
          });
        cargoClippyWindowsRelease = craneLibCross.cargoClippy (commonArgs {debug = false;}
          // {
            cargoArtifacts = cargoArtifactsWindowsRelease;
          });
      in {
        checks = {
          build = self.packages.${system}.default;

          git-hooks = inputs.git-hooks.lib.${system}.run {
            src = ./.;
            enabledPackages = with pkgs; [
              pkg-config
              systemd
            ];
            hooks = {
              alejandra = {
                enable = true;
                package = pkgs.alejandra;
                args = ["-q"];
              };
              deadnix = {
                enable = true;
                args = ["-e" "-q"];
              };
              statix = {
                enable = true;
                args = ["fix"];
              };

              check-json.enable = true;
              pretty-format-json.enable = true;

              check-toml.enable = true;
              taplo = {
                enable = true;
                package = pkgs.taplo;
              };

              trim-trailing-whitespace.enable = true;
              check-merge-conflicts.enable = true;
            };
          };
        };

        packages = {
          default = craneLib.buildPackage (commonArgs {debug = false;}
            // {
              cargoArtifacts = cargoClippyRelease;

              postInstall = ''
                wrapProgram $out/bin/claws \
                  --prefix LD_LIBRARY_PATH : ${libPath}
              '';
            });
          debug = craneLib.buildPackage (commonArgs {debug = true;}
            // {
              cargoArtifacts = cargoArtifactsDebug;

              postInstall = ''
                wrapProgram $out/bin/claws \
                  --prefix LD_LIBRARY_PATH : ${libPath}
              '';
            });
          windows = craneLibCross.buildPackage (windowsArgs {debug = false;}
            // {
              cargoArtifacts = cargoClippyWindowsRelease;
            });
          windowsDebug = craneLibCross.buildPackage (windowsArgs {debug = true;}
            // {
              cargoArtifacts = cargoClippyWindowsDebug;
            });
        };

        formatter = pkgs.alejandra;

        devShells.default = pkgs.callPackage ./nix/shell.nix {inherit self inputs craneLib;};
      }
    );
}
