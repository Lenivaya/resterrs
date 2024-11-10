<h1 align="center">resterrs 😴</h1>
<p align="center">Simple linux service for giving other services or apps some rest based on power conditions</p>

<div align="center">

![CI](https://github.com/Lenivaya/qrrs/workflows/CI/badge.svg)
[![License](https://img.shields.io/github/license/lenivaya/resterrs)](./LICENSE)

![monkey sleeping](https://github.com/user-attachments/assets/1fd2f02b-a229-48f1-9827-2a389b9c1ef5)

</div>

<details>
<summary>Table of contents</summary>

- [Use cases](#use-cases)
- [Use cases explanation](#use-cases-explanation)
- [Usage](#usage)
  - [NixOS](#nixos)
- [Configuration](#configuration)
- [Configuration examples](#configuration-examples)

</details>

# Use cases

- [x] Managing of user services
  - automatic start/stop of user services based on power conditions
- [x] Managing of system services
  - automatic start/stop of system services based on power conditions
- [x] Managing of user apps
  - automatic kill of apps based on power conditions
- [x] Managing of arbitrary commands
  - running commands based on power conditions

_By using word services I mean systemd services_

## Use cases explanation

**_A good [config example](#configuration-examples) is worth a thousand words._**

This application was born from my specific workflow needs. I use a laptop with multiple services and applications that are connected to external monitors and power. In this setup, I want everything running at full capacity.

However, there's often a case where I unplug my laptop, or it's unplugged by other means (power shortages due to war), when that happens I definitely wouldn't want to do manual work stopping services I don't need while running on battery. Some examples include:

- **deskflow**: For sharing mouse/keyboard between computers
- **syncthing**: For home network file sharing (meaningless when not at home)
- **picom**: For desktop compositing and pretty shadows/animations
- **easyeffects**: For audio processing
- And any other services that can be automatically managed based on power conditions

While you can manage these services manually, automation is far more efficient. The traditional approach uses udev rules:

```shell
SUBSYSTEM=="power_supply",ENV{POWER_SUPPLY_ONLINE}=="0",RUN+="/bin/killcompton.sh"
SUBSYSTEM=="power_supply",ENV{POWER_SUPPLY_ONLINE}=="1",RUN+="/bin/startcompton.sh"
```

However, this solution is cumbersome to maintain and lacks user-friendly configuration options. So let's get our hands dirty and write some things on rust.

# Usage

## NixOS

To start using this service on NixOS, you must add this repo as input to your flake:

```nix
{
  inputs = {
    resterrs = {
      url = "github:Lenivaya/resterrs";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  # ...
}

```

then import nixos module from this flake:

```nix
{
  imports = [
    inputs.resterrs.nixosModules.default
  ];
}

```

and then you can configure it according to the [configuration in nix section](#configuration-in-nix)

# Configuration

## Configuration examples

### Configuration in TOML

config.toml

```toml
system_services_to_stop = [
    "fwupd",
    "syncthing",
    "bpftune"
]
user_services_to_stop = [
    "kdeconnect",
    "picom",
    "easyeffects"
]
apps_to_stop = [
    "telegram-desktop",
    "vesktop",
    "easyeffects"
]
commands_unplugged = [
    "bluetoothctl power off"
]
commands_plugged = [
    "bluetoothctl power on"
]
username = "leniviy"
```

### Configuration in Nix

```nix
{
  services.resterrs = {
    enable = true;
    settings = {
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
    };
    extraServicePackages = with pkgs; [
      bluez
    ];
  };
}

```
