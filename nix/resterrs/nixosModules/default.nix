perSystem:
{
  config,
  pkgs,
  lib,
  ...
}:
with lib;
let
  cfg = config.services.resterrs;

  inherit (lib.meta) getExe';
  inherit (lib.options) mkEnableOption mkPackageOption mkOption;

  toTOML = pkgs.formats.toml { };
in
{
  options.services.resterrs = {
    enable = mkEnableOption "Resterrs, simple rester";
    package = mkPackageOption pkgs "resterrs" { } // {
      default = perSystem.config.packages.resterrs;
    };
    extraServicePackages = mkOption {
      type = types.listOf types.package;
      default = [ ];
      example = literalExpression ''
        [ pkgs.curl ]
      '';
      description = ''
        Additional packages to be installed with resterrs systemd unit.
        (needed because systemd doesn't expose user PATH to it's units)
      '';
    };
    serviceLogLevel = mkOption {
      type = types.str;
      default = "error,info";
      example = "error,info,debug,trace";
      description = "Rust log level for the resterrs service";
    };
    settings = mkOption {
      type = toTOML.type;
      default = { };
      example = literalExpression ''
        system_services_to_stop = [
          "fwupd"
          "syncthing"
          "bpftune"
        ];
        user_services_to_stop = [
          "kdeconnect"
          "picom"
          "easyeffects"
        ];
        apps_to_stop = [
          "telegram-desktop"
          "vesktop"
          "deskflow"
        ];
        commands_unplugged = [
          "bluetoothctl power off"
        ];
        commands_plugged = [
          "bluetoothctl power on"
        ];
        username = config.user.name;
      '';
      description = "TOML configuration for Resterrs. See the Resterrs documentation for available options.";
    };
  };

  config = mkIf cfg.enable {
    systemd.services.resterrs =
      let
        settings' = toTOML.generate "resterrs-config.toml" cfg.settings;
      in
      {
        description = "Resterrs Service, rests your apps and services when you plug or unplug your device";
        wantedBy = [ "multi-user.target" ];
        path = cfg.extraServicePackages;
        script = "${getExe' cfg.package "resterrs"} --log-driver journald -c ${settings'}";
        environment = {
          RUST_LOG = cfg.serviceLogLevel;
        };
        serviceConfig = {
          Restart = "always";
          RestartSec = "1";
          User = "root";
          DynamicUser = true;
          StateDirectory = "resterrs";
          ConfigurationDirectory = "resterrs";
          RuntimeDirectory = "resterrs";
        };
      };
  };
}
