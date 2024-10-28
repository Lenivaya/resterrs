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

    settings = mkOption {
      type = toTOML.type;
      default = { };
      example = literalExpression ''
        system_services_to_stop = ["bpftune", "syncthing", "fwupd"]
        user_services_to_stop = ["kdeconnect", "picom"]
        apps_to_stop = ["telegram-desktop", "vesktop"]
        commands_unplugged = [
          "bluetoothctl power off"
        ]
        commands_plugged = [
          "bluetoothctl power on"
        ]
        username = "leniviy"
      '';
      description = "TOML configuration for Resterrs. See the Resterrs documentation for available options.";
    };
  };

  config = mkIf cfg.enable {
    systemd.services.resterrs = {
      description = "Resterrs Service";
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];
      serviceConfig = {
        ExecStart = "${getExe' cfg.package "resterrs"} -c ${toTOML.generate "resterrs-config.toml" cfg.settings}";
        Restart = "always";
        RestartSec = "10";
        User = "root";
        DynamicUser = true;
        StateDirectory = "resterrs";
        ConfigurationDirectory = "resterrs";
        RuntimeDirectory = "resterrs";
      };
    };
  };
}
