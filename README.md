# Kaguya

## Description

![icon](images/icon.png)

Linux game saves and configurations backup manager for CLI enjoyers.

- Icon Picture: [3：00 am](https://www.pixiv.net/artworks/138257419)
- Icon Author: [顽咲\_Little](https://www.pixiv.net/users/97933976)
- Project name inspired by: [Kaguya Houraisan - Touhou WiKi](https://en.touhouwiki.net/wiki/Kaguya_Houraisan)

## Quick Start

- Default global config: `$XDG_CONFIG_HOME/kaguya/config.toml`
- Default vault dir: `$XDG_DATA_HOME/kaguya/vault`
- Default vault config: `$XDG_DATA_HOME/kaguya/vault/vault.toml`

```shell
# Add a game to kaguya vault config
kaguya config add --id game-a --paths test-games/game-a/config.json

# List all the games in vault config
kaguya config list [-l/--long]

# Backup action
kaguya vault backup

# Restore
# Use latest version if '--version' is not provided
kaguya vault restore --id <ID> [--version <VERSION>] [--paths <PATH1> [<PATH2>...]]
```

## Installation

### From source

#### From `crate.io`

```shell
cargo install kaguya
```

#### From local source

```shell
git clone https://github.com/AllenWu233/kaguya
cd kaguya
cargo install --path .
```

## Uninstallation

```shell
cargo uninstall kaguya
```

## Todo

- [ ] Implement backup pruning mechanism
- [ ] Global configuration support (config.toml)
- [ ] Additional compression formats (e.g., .zip)
- [ ] TUI interface (kaguya-tui)
- [ ] Auto-discovery for Steam and Epic games
