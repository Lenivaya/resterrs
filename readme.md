<h1 align="center">resterrs 😴</h1>
<p align="center">Simple linux service for giving other services or apps some rest based on power conditions</p>

<div align="center">

![CI](https://github.com/Lenivaya/qrrs/workflows/CI/badge.svg)
[![License: MIT](https://img.shields.io/github/license/lenivaya/resterrs)](./LICENSE)

![monkey sleeping](https://github.com/user-attachments/assets/1fd2f02b-a229-48f1-9827-2a389b9c1ef5)

</div>

<details>
<summary>Table of contents</summary>

- [Use cases](#use-cases)
- [Features](#features)
- [Usage](#usage)
- [Configuration](#configuration)
- [License](#license)

</details>

# Use cases

**_A good [config example](#configuration-example) is worth a thousand of words._**

Basically, the need for this application came from the fact of specific conditions that I have in my workflow. I use a laptop with a lot of different services and applications connected to the power source and monitors. In this condition I'd definitely like to have all my services and applications running.

But there's often a case where I unplug my laptop, or it's unplugged by other means (power shortages due to war), when that happens I definitely wouldn't want to do some manual work by stopping services I don't need when running on battery. And there are many of those, something like deskflow for sharing mouse and keyboard between multiple computers, home network file sharing syncthing which is meaningless when I'm not at home, or many-many others like picom, easyeffects, and any other thing that can be automatically disabled/enabled based on power conditions.

Disabling and enabling such things manually is a pain in the ass, so it's definitely better to have some minimal automation for it. Based on the first-glance research, something similar can [be achieved](https://superuser.com/q/1417292) through writing udev rules:

```shell
SUBSYSTEM=="power_supply",ENV{POWER_SUPPLY_ONLINE}=="0",RUN+="/bin/killcompton.sh"
SUBSYSTEM=="power_supply",ENV{POWER_SUPPLY_ONLINE}=="1",RUN+="/bin/startcompton.sh"
```

But that's also too cumbersome and not quite user-friendly nor extensible. So let's get our hands dirty and write some things on rust.

# Features

- [x] Managing of user services
- [x] Managing of system services
- [x] Managing of user apps
- [x] Managing of arbitrary commands

_By using word services I mean systemd services_

# Configuration

## Configuration examples

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

or as I'm doing that with nix:

```nix
{
  services.resterrs = enabled // {
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
      bluez-experimental
    ];
  };
}
```

