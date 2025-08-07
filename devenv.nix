{
  pkgs,
  lib,
  config,
  # inputs,
  ...
}: let
  libPath = with pkgs;
    lib.makeLibraryPath [
      libGL
      libxkbcommon

      wayland

      xorg.libX11
      xorg.libXcursor
      xorg.libXi
      xorg.libXrandr
    ];
in {
  # https://devenv.sh/basics/
  # env.GREET = "devenv";

  # https://devenv.sh/packages/
  packages = with pkgs; [
    tailwindcss_4
    watchman

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

    auto-patchelf
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
      directory = "frontend";
    };
    typescript.enable = true;
  };

  # env.LD_LIBRARY_PATH = libPath;

  # https://devenv.sh/processes/
  # processes.cargo-watch.exec = "cargo-watch";

  # https://devenv.sh/services/
  services = {
    nginx = {
      enable = true;
      httpConfig = ''
        server {
          listen 8080;
          # server_name your-domain.com;

          root ${config.env.DEVENV_ROOT}/frontend/dist;
          index index.html;

          location / {
            try_files $uri $uri/ =404;
          }

          # location ~* \.(css|js|png|jpg|jpeg|gif|ico|svg)$ {
          #   expires 1y;
          #   add_header Cache-Control "public, immutable";
          # }
        }
      '';
    };
  };

  # https://devenv.sh/scripts/
  # scripts.hello.exec = ''
  #   echo hello from $GREET
  # '';
  #
  # enterShell = ''
  #   hello
  #   git --version
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
    rustfmt.enable = true;
    clippy.enable = true;
    treefmt = {
      enable = true;
      settings = {
        formatters = with pkgs; [
          alejandra
          deadnix
        ];
      };
    };
  };

  # See full reference at https://devenv.sh/reference/options/
}
