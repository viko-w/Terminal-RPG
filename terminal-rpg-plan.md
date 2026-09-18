# Terminal RPG --- Project Plan

## Concept

Build a persistent RPG system that lives inside the terminal and turns
normal computer usage into game progression.

The goal is **not** to make a game that you launch separately. The RPG
should quietly integrate with the shell:

``` text
$ git commit -m "fix"

⚔ COMMAND COMPLETE

Git Apprentice
+15 XP
Git +1
```

Over time the player gets:

-   XP
-   levels
-   skills/stats
-   achievements
-   quests
-   streaks
-   unlocks
-   statistics/history
-   fun reactions to commands and events

The system should reward successful work rather than encouraging
pointless command spam.

------------------------------------------------------------------------

# 1. Language / Technology Choice

## Recommendation: Rust

Use **Rust** for the core application.

### Why Rust?

This project is an unusually good fit for Rust because the final product
should ideally be:

-   a single executable
-   fast to start
-   lightweight
-   easy to call from shell hooks
-   able to inspect the local system
-   cross-platform where practical
-   fun to customize and extend

A Rust binary means the user can eventually have:

``` bash
rpg
rpg status
rpg quests
rpg achievements
rpg history
```

without needing a Python virtual environment or interpreter at runtime.

It also makes the project itself a useful Rust-learning project.

### Why not Python?

Python would be faster for the first prototype and is completely capable
of building this.

However, the RPG is intended to become a small permanent piece of the
user's development environment. Rust's single-binary distribution and
low startup overhead make it a better long-term fit.

### Why not Go?

Go would also be an excellent choice, especially for CLI development.
Rust gets the nod here primarily because the project is also an
opportunity to build something interesting in Rust and eventually
experiment with terminal UIs, filesystem/event handling, and a polished
CLI.

------------------------------------------------------------------------

# 2. Initial Architecture

Keep the architecture simple.

``` text
terminal-rpg/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── player.rs
│   ├── xp.rs
│   ├── achievements.rs
│   ├── quests.rs
│   ├── commands.rs
│   ├── events.rs
│   └── storage.rs
├── data/
│   └── achievements.toml
└── shell/
    ├── zsh.sh
    ├── bash.sh
    └── fish.fish
```

Do **not** build a huge framework at the beginning.

Start with a working CLI and add systems one at a time.

------------------------------------------------------------------------

# 3. Player Data

The RPG needs persistent state.

Example:

``` text
Level: 12
XP: 1284 / 1520
Total XP: 18432

Stats:
  Command: 17
  Git: 19
  Docker: 11
  Linux: 24
  Coding: 13

Achievements:
  First Blood
  Container Tamer
  Night Shift

Current streak: 6 days
```

Store this somewhere appropriate for the OS.

On Linux, something like:

``` text
~/.local/share/terminal-rpg/
```

is a good target.

Potential file:

``` text
player.toml
```

TOML is a nice fit because it is human-readable and easy to
inspect/edit.

------------------------------------------------------------------------

# 4. CLI

The first version should provide:

``` bash
rpg
rpg status
rpg xp <amount>
rpg quests
rpg achievements
rpg stats
rpg history
```

Example:

``` text
$ rpg status

╔══════════════════════════════════════╗
║            TERMINAL HERO             ║
╠══════════════════════════════════════╣
║ Level 12                             ║
║ XP     1284 / 1520                   ║
║                                      ║
║ ⚔ Command       17                   ║
║ 🌿 Git           19                   ║
║ 🐳 Docker        11                   ║
║ 🐧 Linux         24                   ║
║ 💻 Coding        13                   ║
╚══════════════════════════════════════╝
```

------------------------------------------------------------------------

# 5. XP System

Two XP tracks:

-   **Command XP** — every successful command pays a small flat amount
    (e.g. 5 XP). Deliberately tiny: farming `ls` 100 times is 500 XP,
    less than one good achievement.
-   **Event XP** — meaningful events (commits, builds, achievements)
    pay the real XP.

Initial examples (defaults -- every value is configurable):

  Event                                  XP
  -------------------------- --------------
  Basic successful command                5
  Git commit                             15
  Git push                               20
  Docker build                           20
  Docker Compose up                      25
  Successful test suite                  25
  Creating a script                      15
  Fixing a failed build                  50
  Achievement tier                       20--500, scales with level

Command XP is flat and capped at a trickle, so spamming commands is
never a real farming strategy -- the interesting XP comes from events.

The system should eventually detect meaningful events rather than simply
farming XP by typing:

``` bash
ls
ls
ls
ls
ls
```

------------------------------------------------------------------------

# 6. Leveling

Use an XP curve rather than a fixed amount per level.

Possible starting formula:

``` text
XP required = 100 × level^1.5
```

The exact formula can be tuned later.

Level-ups should produce an obvious terminal notification:

``` text
✨ LEVEL UP!

You are now level 13!

+1 Linux
+1 Command
```

------------------------------------------------------------------------

# 7. Stats / Skills

Possible skills:

``` text
Command
Linux
Git
Docker
Coding
Networking
Shell
DevOps
```

Commands/events contribute to relevant skills.

For example:

``` text
git commit
    → Git +1

docker build
    → Docker +1
    → DevOps +1

ssh server
    → Networking +1
    → Linux +1

cargo build
    → Coding +1
```

Stats should be descriptive rather than pretending to measure actual
technical ability.

------------------------------------------------------------------------

# 8. Shell Integration

This is one of the most important parts.

The RPG should integrate with:

-   Bash
-   Zsh
-   Fish

The shell integration captures useful information about commands.

Important:

**Only reward successful commands by default.**

For example:

``` text
$ git commit -m "hello"

⚔ COMMAND COMPLETE

+15 XP
Git +1
```

But:

``` text
$ git commit

✗ COMMAND FAILED

No XP awarded.
```

The shell hook should pass information such as:

-   command
-   exit code
-   current directory
-   timestamp
-   relevant environment information

Avoid sending command contents anywhere. This should remain entirely
local.

------------------------------------------------------------------------

# 9. Command Detection

Create a command/event system instead of hardcoding everything into the
shell hooks.

Conceptually:

``` text
Shell
  ↓
RPG event
  ↓
Event matcher
  ↓
Reward
  ↓
Player state
```

Example:

``` text
git commit
    ↓
GitCommit event
    ↓
+15 XP
Git +1
```

This makes it possible to add new commands later without rewriting the
shell integration.

------------------------------------------------------------------------

# 10. Achievements

Achievements should be persistent and event-based.

### Tiers

Achievements have levels instead of being one-shot unlocks:

``` text
🐺 Git Gud              Level 3 / 5
   Lv 1: 10 git commits        +20 XP
   Lv 2: 50 git commits        +50 XP
   Lv 3: 200 git commits      +120 XP
   Lv 4: 500 git commits      +250 XP
```

Each tier needs more than the last. Tier XP scales with the player's
current level -- early game a tier pays ~20 XP, late game up to 500+.

The engine only implements the logic. Tier names, threshold counts,
XP values, how many tiers an achievement has, and the achievement list
itself are all data, defined in `data/achievements.toml` and user-
editable. The examples below are just defaults:

Good tier candidates: "Git Gud" (git events), "Container Tamer"
(docker), "Night Owl" (late-night terminal use), "Keyboard Warrior"
(total commands).

### First Blood

Make your first Git commit.

Reward:

``` text
+20--100 XP (default; scales with player level, value configurable)
```

### Container Tamer

Run your first Docker container.

### Night Shift

Use the terminal after midnight.

### Diplomatic Incident

Encounter a Git merge conflict.

### With Great Power

Use `sudo`.

### It Works On My Machine

Successfully complete a deployment-related event.

### Archaeologist

Open a project you haven't touched in a year.

### Touch Grass

Go an unusually long period without terminal activity.

The last one should probably have a hidden implementation.

------------------------------------------------------------------------

# 11. Quests

Add daily/weekly/random quests later.

Example:

``` text
DAILY QUESTS
────────────────────────────

⚔ Commit 3 times
   ███████░░░ 2 / 3

🐳 Start 2 containers
   █████░░░░░ 1 / 2

🐧 Use 5 different Linux commands
   ████████░░ 4 / 5
```

Quest completion gives XP and possibly achievements.

Avoid quests that encourage destructive or meaningless behavior.

------------------------------------------------------------------------

# 12. Event Types

Eventually define a general event model.

Possible events:

``` text
CommandSucceeded
CommandFailed
GitCommit
GitPush
GitMergeConflict
DockerStarted
DockerBuilt
DockerStopped
TestPassed
TestFailed
PackageInstalled
SshConnection
FileCreated
FileDeleted
BuildSucceeded
BuildFailed
SystemBoot
LateNightActivity
FirstUse
```

Not every event needs to award XP.

Some can only trigger achievements.

------------------------------------------------------------------------

# 13. Streaks

Track things like:

``` text
Current streak: 6 days
Longest streak: 21 days
```

Potentially track:

-   daily terminal activity
-   weekly project activity
-   Git activity

Do not make streaks overly punishing.

Missing a day should not destroy months of progression.

------------------------------------------------------------------------

# 14. History

Add:

``` bash
rpg history
```

Example:

``` text
RECENT EVENTS
────────────────────────────────

23:04  Git commit             +15 XP
22:51  Docker build           +20 XP
22:12  Achievement unlocked   +50 XP
21:48  Python test suite      +25 XP
```

This will make the system feel much more like an actual RPG.

------------------------------------------------------------------------

# 15. Terminal UI

Once the underlying system works, consider a TUI.

Potential command:

``` bash
rpg ui
```

Possible screens:

``` text
┌─────────────────────────────────────────────┐
│ TERMINAL HERO             LEVEL 12          │
├─────────────────────────────────────────────┤
│                                             │
│ XP       █████████████░░░  1284 / 1520      │
│                                             │
│ Git          19                            │
│ Linux        24                            │
│ Docker       11                            │
│ Coding       13                            │
│                                             │
├─────────────────────────────────────────────┤
│ QUESTS                                      │
│                                             │
│ ✓ First Blood                              │
│ ✓ Container Tamer                          │
│ □ Survive a merge conflict                 │
│ □ Build something in Rust                  │
└─────────────────────────────────────────────┘
```

A Rust TUI library such as `ratatui` can be evaluated when this stage is
reached.

Do not start here.

------------------------------------------------------------------------

# 16. Notifications

Make events entertaining but not annoying.

Good:

``` text
⚔ COMMAND COMPLETE
+15 XP
```

Occasional:

``` text
🏆 ACHIEVEMENT UNLOCKED

NIGHT SHIFT

+100 XP
```

Bad:

``` text
+1 XP
```

after every `cd`.

The system should have configurable notification verbosity:

``` bash
rpg config notifications minimal
rpg config notifications normal
rpg config notifications chaotic
rpg config notifications off
```

------------------------------------------------------------------------

# 17. Easter Eggs

Add hidden achievements and rare events.

Examples:

``` text
"Why Are You Awake?"
Use the terminal at 04:00.

"Dependency Hell"
Have a package manager fail repeatedly.

"Here We Go Again"
Run the same failed command 10 times.

"Do Not Question The Architecture"
Have a project containing an absurdly complicated shell script.

"Production Is Down"
A deliberately fictional achievement triggered by certain local/dev events.
```

The system should never encourage attacking systems, deleting important
data, or otherwise doing dangerous things for achievements.

------------------------------------------------------------------------

# 18. Configuration

Eventually support:

``` text
~/.config/terminal-rpg/config.toml
```

Possible settings:

``` toml
notifications = "normal"
sound = false
colors = true
daily_quests = true
track_shell_commands = true
```

Allow users to disable specific event categories.

Achievement definitions (names, requirements, tiers, rewards, XP
values) stay in `data/achievements.toml`, not in code. The engine
implements generic logic: match event counters against config-defined
thresholds, award config-defined rewards.

------------------------------------------------------------------------

# 19. Privacy / Safety Design

This should be **local-first**.

No cloud backend is needed.

Do not transmit:

-   commands
-   filenames
-   project names
-   environment variables
-   Git remotes
-   passwords
-   tokens
-   SSH information

Shell history can contain sensitive information, so the integration
should avoid storing raw commands unless explicitly enabled.

Prefer storing normalized events:

``` text
GitCommit
DockerBuild
CommandFailed
```

rather than:

``` text
git commit -m "my secret project..."
```

------------------------------------------------------------------------

# 20. Development Milestones

## Milestone 1 --- Character Sheet

Build:

``` bash
rpg
rpg status
rpg xp 50
```

Features:

-   persistent player
-   XP
-   levels
-   basic stats

------------------------------------------------------------------------

## Milestone 2 --- Real Events

Add:

``` bash
rpg event git_commit
rpg event docker_build
```

Events grant XP and stats.

------------------------------------------------------------------------

## Milestone 3 --- Shell Integration

Add Bash/Zsh/Fish hooks.

Automatically detect successful commands.

------------------------------------------------------------------------

## Milestone 4 --- Achievements

Implement:

``` bash
rpg achievements
```

Add 10--20 achievements.

------------------------------------------------------------------------

## Milestone 5 --- Quests

Implement:

``` bash
rpg quests
```

Add daily and weekly quests.

------------------------------------------------------------------------

## Milestone 6 --- History

Implement:

``` bash
rpg history
```

Store a local event history.

------------------------------------------------------------------------

## Milestone 7 --- Polish

Add:

-   pretty output
-   colors
-   progress bars
-   better messages
-   configuration
-   installation script

------------------------------------------------------------------------

## Milestone 8 --- TUI

Build:

``` bash
rpg ui
```

Use a Rust terminal UI library.

------------------------------------------------------------------------

## Milestone 9 --- Deep Integrations

Potential integrations:

-   Git
-   Docker
-   Cargo
-   npm
-   Python
-   SSH
-   systemd
-   package managers
-   test runners
-   editors

Only add integrations when they provide genuinely useful events.

------------------------------------------------------------------------

# 21. Future Crazy Ideas

Once the basic project is stable:

### Classes

Choose a class based on activity:

``` text
⚔ Systems Knight
🐳 Container Mage
🌿 Git Ranger
💻 Code Wizard
```

Avoid locking the player permanently into a class.

### Titles

Unlock titles:

``` text
Level 5  → Terminal Apprentice
Level 10 → Shell Adept
Level 20 → Command Knight
Level 50 → Kernel Whisperer
```

### Loot

Commands can very rarely produce cosmetic loot:

``` text
✨ Found: Rusty USB Stick
✨ Found: Ancient Config File
✨ Found: Legendary Mechanical Keyboard
```

No real-world value; purely cosmetic.

### Bosses

Long-running goals could become bosses:

``` text
BOSS: THE LEGACY CODEBASE

████████████████░░░░ 82%

Defeat conditions:
✓ Understand the architecture
✓ Fix 5 bugs
□ Remove one TODO from 2019
□ Add tests
```

### Prestige

At very high levels:

``` text
rpg prestige
```

resets levels while preserving achievements/cosmetics.

This should only be considered much later.

------------------------------------------------------------------------

# 22. Guiding Principle

The most important design rule:

> **The RPG should reward things you already wanted to do, not make you
> do pointless things to grind XP.**

Good:

``` text
You fixed your build → XP
You made a commit → XP
You finished a project → XP
You learned a new tool → achievement
```

Bad:

``` text
Run `ls` 100 times → 500 XP trickle (command XP stays tiny)
Delete a file → XP
Run random commands → XP
Keep terminal open for 14 hours → massive XP
```

The game should make normal development feel more satisfying without
getting in the way.

------------------------------------------------------------------------

# 23. First Build Target

The first version should be deliberately tiny.

After installation:

``` bash
rpg
```

should work.

Then:

``` bash
rpg xp 50
```

should give XP.

Then:

``` bash
rpg status
```

should show the character.

Once that works, **stop and make a Git commit**.

The RPG has officially become self-aware.
