<div align="center">

```
    █████   ██  ██
   ██   ██  ██  ██
   ███████  ██████
   ██   ██  ██  ██
   ██   ██  ██  ██ .sh
```

# ah.sh

_Magic development environments powered by Nix_

[![CI](https://img.shields.io/github/actions/workflow/status/z1-0/ah.sh/ci.yml?style=flat-square)](https://github.com/z1-0/ah.sh/actions)
[![Release](https://img.shields.io/github/v/release/z1-0/ah.sh?style=flat-square)](https://github.com/z1-0/ah.sh/releases)
[![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](LICENSE)

[Features](#features) • [Installation](#installation) • [Quick Start](#quick-start) • [Usage](#usage) • [Configuration](#configuration)

</div>

Spin up reproducible, per-project development shells in one command. `ah` generates a Nix flake on the fly, manages sessions so you can restore them instantly, and supports **60+ languages** out of the box — no `flake.nix` authoring required.

```bash
# Need Rust and Go? Done.
ah rust go
```

## Features

- **Instant dev shells** — type `ah rust` and you're in a shell with the Rust toolchain ready
- **Session management** — sessions are persisted and can be listed, restored, updated, or removed
- **Multiple providers** — choose between [devenv](https://devenv.sh) (59 languages) and [dev-templates](https://github.com/the-nix-way/dev-templates) (46 templates)
- **Language aliases** — use `py` instead of `python`, `ts` instead of `typescript`, `cpp` instead of `cplusplus`
- **Directory history** — `ah restore` remembers which sessions you used in each directory
- **Nix profiles** — subsequent shell entries are near-instant thanks to cached Nix profiles
- **Shell completions** — dynamic completions for Bash, Zsh, Fish, Elvish, and PowerShell
- **Configurable** — TOML config with environment variable overrides

## Installation

> [!TIP]
> Configure the **[Binary cache](#binary-cache)** before installing to avoid compiling the project from source.

### Prerequisites

[Nix](https://nixos.org) with Flakes enabled is required. If you don't have it installed, we recommend using the [Determinate Nix Installer](https://install.determinate.systems/).

### Run without installing

```bash
nix run github:z1-0/ah.sh -- rust go
```

### Imperative (Nix Profile)

```bash
nix profile install github:z1-0/ah.sh
```

### Declarative (Nix Flake)

Add to your `flake.nix` inputs:

```nix
inputs.ah.url = "github:z1-0/ah.sh";
```

Then add it to your system or home packages:

```nix
environment.systemPackages = [
  inputs.ah.packages.${system}.default
];
```

### Binary Cache

Pre-built binaries are available via [Cachix](https://app.cachix.org). Choose **one** of the following options based on your workflow:

#### Option A: Imperative (CLI)

If you use profile/CLI packages, run:

```bash
cachix use z1-0
```

#### Option B: Declarative (NixOS / Home Manager)

If you configure NixOS or Home Manager via code, add this to your settings:

```nix
nix.settings = {
  substituters = [ "https://z1-0.cachix.org" ];
  trusted-public-keys = [ "z1-0.cachix.org-1:e4TgPqNGXlI7xxs73HxTE65qUjmWaPxwnJX2Qk4Ng5U=" ];
};
```

## Quick Start

```bash
# Create a Python dev shell
ah python

# Combine multiple languages
ah rust go nodejs

# You're now inside a Nix develop shell with all tools available!
```

## Usage

### Creating dev shells

```bash
# Shorthand — languages as positional args
ah rust go

# Explicit subcommand
ah use rust go

# Specify a provider
ah use python --provider devenv
```

### Session management

```bash
# List all sessions
ah session list

# Restore a session by index or ID
ah session restore 1
ah session restore a3f8c2d1

# Restore from directory history (interactive)
ah restore

# Update session dependencies (re-runs nix flake update)
ah session update
ah update 1

# Remove sessions
ah session remove 1 2
ah session remove a3f8c2d1

# Clear all sessions
ah session clear
```

### Providers

```bash
# List available providers
ah provider list

# Show supported languages for a provider
ah provider show devenv
ah provider show dev-templates
```

### Shell completions

```bash
# Generate completions for your shell
ah completion bash
ah completion zsh
ah completion fish
```

> [!TIP]
> Add `source <(ah completion bash)` (or equivalent for your shell) to your shell profile for persistent completions. Language names and session IDs are completed dynamically.

### Command aliases

| Alias            | Equivalent           |
| ---------------- | -------------------- |
| `ah <languages>` | `ah use <languages>` |
| `ah restore`     | `ah session restore` |
| `ah update`      | `ah session update`  |

## Configuration

`ah` creates a config file at `~/.config/ah/config.toml` on first run:

```toml
# Provider type: devenv or dev-templates
provider = "dev-templates"

# Leave empty to use the $SHELL environment variable
# shell = "zsh"
```

All settings can be overridden via environment variables with the `AH_` prefix:

```bash
AH_PROVIDER=devenv ah rust
```

> [!NOTE]
> The config schema is available at [`src/assets/config.schema.json`](src/assets/config.schema.json).

## Supported Languages

<details>
<summary><strong>devenv</strong> — 59 languages</summary>

ansible, c, clojure, cplusplus (cpp), crystal, cue, dart, deno, dotnet, elixir, elm, erlang, fortran, gawk, gleam, go, hare, haskell, helm, idris, java, javascript (js), jsonnet, julia, kotlin, lean4, lobster, lua, nim, nix, ocaml, odin, opentofu, pascal, perl, php, pkl, purescript, python (py), r, racket, raku, robotframework, ruby, rust, scala, shell (sh, bash), solidity, standardml, swift, terraform, texlive, typescript (ts), typst, unison, v, vala, zig

</details>

<details>
<summary><strong>dev-templates</strong> — 46 templates</summary>

bun, c-cpp, clojure, csharp, cue, deno, dhall, elixir, elm, empty, gleam, go, hashi, haskell, haxe, java, jupyter, kotlin, latex, lean4, nickel, nim, nix, node (js, javascript, nodejs), ocaml, odin, opa, php, platformio, powershell, presenterm, protobuf, pulumi, purescript, python (py), r, ruby, rust, scala, shell (sh, bash), swi-prolog, swift, typst, vlang, zig

</details>

## How It Works

1. You run `ah rust go` — `ah` resolves language aliases and selects the provider
2. If a matching session already exists, it restores the cached Nix profile instantly
3. Otherwise, `ah` generates a `flake.nix` tailored to your languages, builds it, and drops you into a `nix develop` shell
4. Session metadata is persisted so you can list, restore, update, or remove it later
5. Directory history tracks which sessions you used where, enabling quick `ah restore` lookups
