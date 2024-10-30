<h1 align="center">resterrs 😴</h1>
<p align="center">Simple linux service for giving other services or apps some rest based on power conditions</p>

<div align="center">

![CI](https://github.com/Lenivaya/qrrs/workflows/CI/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)
<a href="https://crates.io/crates/qrrs"><img src="https://img.shields.io/crates/v/qrrs.svg?colorB=319e8c" alt="Version info"></a><br>

![monkey sleeping](https://github.com/user-attachments/assets/0c984728-94f4-481a-832d-68514cf530d1)

</div>

<details>
<summary>Table of contents</summary>

- [Use cases][#use-cases]
- [Features](#features)
- [Usage](#usage)
- [Configuration](#configuration)
- [License](#license)

</details>

# Use cases

**_A good [config example](#configuration-example) is worth a thousand of words._**

Basically the need for this app came from the fact of specific conditions that I have in my workflow. I use laptop with a lot of different services and apps, connected to the power source and monitors. In that condition I'd definitely like to have all of my services and apps running.

But there's often a case when I'm disconnecting laptop from the power source, or it's being disconnected by other means (power shortages due to the war), when that happens I'd definitely would not want doing some hand work by stopping services that I don't need when running on battery. And there's many of that, something like deskflow for sharing mouse and keyboard between multiple computers, home network file-sharing syncthing which meaningless when I'm not at home, or many-many others like picom, easyeffects and any other things than can be auto disabled/re-enabled based on power conditions.

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
    "fwupd"
    "syncthing"
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
  };
}
```
