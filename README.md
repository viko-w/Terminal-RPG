# terminal-rpg

A persistent RPG that lives in your terminal and turns normal computer
usage into game progression. Reward your shell habits with XP, levels,
and stats.

## Install

```bash
cargo install --path .
```

Rust toolchain required (https://rustup.rs).

## Usage

```bash
rpg                    show character sheet
rpg status             show character sheet
rpg xp <amount>        gain XP
```

Example:

```bash
$ rpg xp 50
+50 XP

$ rpg status
╔══════════════════════════════════════╗
║          TERMINAL HERO               ║
╠══════════════════════════════════════╣
║Level     1                           ║
║XP        50 / 100                    ║
║Total XP  50                          ║
║                                      ║
║  Command 0                           ║
╚══════════════════════════════════════╝
```

## Data

Player state is stored at `~/.local/share/terminal-rpg/player.toml`
(standard TOML, human-readable, safe to edit while the game is not
running).