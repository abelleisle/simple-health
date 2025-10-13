{
  description = "Description for the project";

  inputs = {
    devenv-root = {
      url = "file+file:///dev/null";
      flake = false;
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    devenv.url = "github:cachix/devenv";
    nix2container.url = "github:nlewo/nix2container";
    nix2container.inputs.nixpkgs.follows = "nixpkgs";
    mk-shell-bin.url = "github:rrbutani/nix-mk-shell-bin";
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = inputs @ {
    flake-parts,
    devenv-root,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.devenv.flakeModule
      ];
      systems = ["x86_64-linux" "i686-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin"];

      perSystem = {
        config,
        self',
        inputs',
        pkgs,
        system,
        ...
      }: {
        # Per-system attributes can be defined here. The self' and inputs'
        # module parameters provide easy access to attributes of the same
        # system.

        packages.simple-health = let
          boringssl-wrapper = pkgs.runCommand "boringssl-wrapper" {} ''
            mkdir $out
            cd $out
            ln -s ${pkgs.boringssl.out}/lib build
            ln -s ${pkgs.boringssl.dev}/include include
          '';
        in
          pkgs.rustPlatform.buildRustPackage {
            pname = "simple-health";
            version = "0.1.0";

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            nativeBuildInputs = with pkgs; [
              # Frontend
              tailwindcss_4
              typescript

              # Backend
              pkg-config
              cmake
              perl
              clang
              go
              nasm
              python3
              llvmPackages.libclang
            ];

            buildInputs = with pkgs; [
              openssl
            ];

            env = {
              RUST_BACKTRACE = "full";
              LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
              BORING_BSSL_PATH = "${boringssl-wrapper}";

              # Guess we don't need these
              # OPENSSL_DIR = "${pkgs.openssl.dev}";
              # OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
              # BORING_BSSL_PATH = "${pkgs.boringssl}/lib";
              # BORING_BSSL_INCLUDE_PATH = "${pkgs.boringssl.dev}/include";
            };

            preBuild = ''
              # Build the frontend TypeScript and Tailwind CSS
              cd frontend/web
              tsc
              tailwindcss --output static/css/styles.css
              # bun install --frozen-lockfile
              # bun run build-ts
              cd ../..
            '';

            postInstall = ''
              # Copy the static assets to the share directory
              mkdir -p $out/share/simple-health
              cp -r frontend/web/static $out/share/simple-health/
              cp -r frontend/web/templates $out/share/simple-health/
            '';

            meta = with pkgs.lib; {
              description = "Simple Health application";
              platforms = platforms.all;
            };
          };

        packages.default = self'.packages.simple-health;

        devenv.shells.default = {
          imports = [
            ./devenv.nix
          ];
        };
      };
      flake = {
        # The usual flake attributes can be defined here, including system-
        # agnostic ones like nixosModule and system-enumerating ones, although
        # those are more easily expressed in perSystem.
      };
    };
}
