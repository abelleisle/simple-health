{
  config,
  lib,
  pkgs,
  ...
}:
with lib; let
  cfg = config.services.simple-health;
in {
  options.services.simple-health = {
    enable = mkEnableOption "simple-health server";

    package = mkOption {
      type = types.package;
      default = pkgs.simple-health;
      defaultText = literalExpression "pkgs.simple-health";
      description = "The simple-health package to use.";
    };

    environmentFile = mkOption {
      type = types.nullOr types.path;
      default = null;
      example = "/run/secrets/simple-health.env";
      description = ''
        Path to an environment file that will be loaded by the systemd service.
        This file should contain environment variables in the format KEY=VALUE.

        At minimum, this file should contain:
        - SIMPLE_HEALTH_PG_URL: PostgreSQL connection string (e.g., postgres://user:pass@host/dbname)

        Optional environment variables that can be set in this file:
        - SIMPLE_HEALTH_ADDR: Override the listening address (defaults to the 'address' option)
        - SIMPLE_HEALTH_PORT: Override the listening port (defaults to the 'port' option)

        Note: Variables in the environment file take precedence over the module options.

        The file will be read by systemd's EnvironmentFile directive.
      '';
    };

    user = mkOption {
      type = types.str;
      default = "simple-health";
      description = "User account under which simple-health runs.";
    };

    group = mkOption {
      type = types.str;
      default = "simple-health";
      description = "Group under which simple-health runs.";
    };

    address = mkOption {
      type = types.str;
      default = "localhost";
      example = "0.0.0.0";
      description = ''
        The address on which the simple-health server will listen.
        Use "0.0.0.0" to listen on all interfaces, or "localhost" to listen only on localhost.
      '';
    };

    port = mkOption {
      type = types.port;
      default = 3000;
      example = 8080;
      description = "The port on which the simple-health server will listen.";
    };
  };

  config = mkIf cfg.enable {
    systemd.services.simple-health = {
      description = "Simple Health Server";
      after = ["network.target"];
      wantedBy = ["multi-user.target"];

      serviceConfig = {
        Type = "simple";
        User = cfg.user;
        Group = cfg.group;
        ExecStart = "${cfg.package}/bin/simple-health-server";
        Restart = "on-failure";
        RestartSec = "5s";

        # Load environment variables from file if specified
        EnvironmentFile = mkIf (cfg.environmentFile != null) cfg.environmentFile;

        # Set address and port environment variables
        Environment = [
          "SIMPLE_HEALTH_ADDR=${cfg.address}"
          "SIMPLE_HEALTH_PORT=${toString cfg.port}"
        ];

        # Security hardening
        NoNewPrivileges = true;
        PrivateTmp = true;
        PrivateDevices = true;
        ProtectSystem = "strict";
        ProtectHome = true;
        ProtectKernelTunables = true;
        ProtectKernelModules = true;
        ProtectControlGroups = true;
        RestrictAddressFamilies = ["AF_UNIX" "AF_INET" "AF_INET6"];
        RestrictNamespaces = true;
        LockPersonality = true;
        RestrictRealtime = true;
        RestrictSUIDSGID = true;
        RemoveIPC = true;
        PrivateMounts = true;

        # System call filtering
        SystemCallFilter = ["@system-service" "~@privileged" "~@resources"];
        SystemCallErrorNumber = "EPERM";
      };
    };

    # Create user and group if using default values
    users.users = mkIf (cfg.user == "simple-health") {
      simple-health = {
        description = "Simple Health service user";
        group = cfg.group;
        isSystemUser = true;
      };
    };

    users.groups = mkIf (cfg.group == "simple-health") {
      simple-health = {};
    };
  };
}
