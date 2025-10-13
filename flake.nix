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

        # Equivalent to  inputs'.nixpkgs.legacyPackages.hello;
        # packages.default = pkgs.hello;

        devenv.shells.default = {
          name = "my-project";

          # https://devenv.sh/packages/
          packages = with pkgs; [
            tailwindcss_4
            watchman

            cmake
            clang

            typescript-language-server

            at-spi2-atk
            atkmm
            cairo
            gdk-pixbuf
            glib
            gtk3
            harfbuzz
            librsvg
            libsoup_3
            pango
            webkitgtk_4_1
            openssl
            postgresql

            auto-patchelf
            watchexec
            diesel-cli
            sqlx-cli
          ];

          # https://devenv.sh/languages/
          languages = {
            rust.enable = true;
            javascript = {
              enable = true;
              bun = {
                enable = true;
                install.enable = true;
              };
              directory = "frontend/web";
            };
            typescript.enable = true;
          };

          env = {
            DATABASE_URL = "postgres://gym:membership@localhost/health";
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          };

          # env.LD_LIBRARY_PATH = libPath;

          # https://devenv.sh/processes/
          processes = {
            tailwind = {
              exec = "bun watch-tw";
              cwd = "./frontend/web";
            };
            typescript = {
              exec = "bun watch-ts";
              cwd = "./frontend/web";
            };
            server = {
              # exec = "watchexec -e rs,js,css,tera -r cargo run --bin simple-health-server";
              exec = "watchexec -e rs,tera -r cargo run --bin simple-health-server";
            };
          };

          # https://devenv.sh/services/
          services = {
            postgres = {
              enable = true;
              package = pkgs.postgresql;
              initialDatabases = [
                {
                  name = "health";
                  user = "gym";
                  pass = "membership";
                }
              ];
              listen_addresses = "::";
              extensions = extensions: [
                # extensions.postgis
                # extensions.timescaledb
              ];
              hbaConf = ''
                # Allow the gym user to connect from localhost with password
                local   all             gym                                     md5
                host    all             gym             127.0.0.1/32            md5
                host    all             gym             ::1/128                 md5

                # Default rules for other users
                local   all             all                                     trust
                host    all             all             127.0.0.1/32            trust
                host    all             all             ::1/128                 trust
              '';
              # settings.shared_preload_libraries = "timescaledb";
              # initialScript = "CREATE EXTENSION IF NOT EXISTS timescaledb;";
            };
            # nginx = {
            #   enable = true;
            #   httpConfig = ''
            #     server {
            #       listen 8080;
            #       # server_name your-domain.com;
            #
            #       root ${config.env.DEVENV_ROOT}/frontend/dist;
            #       index index.html;
            #
            #       location / {
            #         try_files $uri $uri/ =404;
            #       }
            #
            #       # location ~* \.(css|js|png|jpg|jpeg|gif|ico|svg)$ {
            #       #   expires 1y;
            #       #   add_header Cache-Control "public, immutable";
            #       # }
            #     }
            #   '';
            # };
          };

          # https://devenv.sh/scripts/
          # scripts.hello.exec = ''
          #   echo hello from $GREET
          # '';
          #
          # enterShell = ''
          # export LIBCLANG_PATH="${pkgs.llvmPackages.libclang}/lib"
          # '';

          # https://devenv.sh/tasks/
          # tasks = {
          #   "myproj:setup".exec = "mytool build";
          #   "devenv:enterShell".after = [ "myproj:setup" ];
          # };

          # https://devenv.sh/tests/
          # enterTest = ''
          #   echo "Running tests"
          #   git --version | grep --color=auto "${pkgs.git.version}"
          # '';

          # https://devenv.sh/git-hooks/
          git-hooks.hooks = {
            clippy = {
              enable = true;
              packageOverrides = with pkgs; {
                inherit cargo clippy;
              };
              extraPackages = with pkgs; [
                openssl
              ];
              settings.allFeatures = true;
            };
            treefmt = {
              enable = true;
              settings = {
                formatters = with pkgs; [
                  # Nix
                  alejandra
                  deadnix
                  # Rust
                  rustfmt
                  # TypeScript/JavaScript
                  nodePackages.prettier
                  # HTML/CSS (prettier handles these too)
                  # Additional formatters can be added here
                ];
              };
            };
          };
        };
      };
      flake = {
        # The usual flake attributes can be defined here, including system-
        # agnostic ones like nixosModule and system-enumerating ones, although
        # those are more easily expressed in perSystem.
      };
    };
}
