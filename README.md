# ah

```
    █████   ██  ██
   ██   ██  ██  ██
   ███████  ██████
   ██   ██  ██  ██
   ██   ██  ██  ██ .sh
```

Magic development environments powered by Nix.

## Overview

`ah` is a CLI tool that declaratively provisions isolated development environments using Nix flakes. It abstracts flake composition behind a language-centric interface, manages environment sessions with persistent state, and supports multiple backend providers.

## Design

### Provider Abstraction

Two providers are available, each with distinct trade-offs:

| Provider        | Backend                              | Languages | Characteristics                        |
| --------------- | ------------------------------------ | --------- | -------------------------------------- |
| `devenv`        | [devenv.sh](https://devenv.sh)       | 58        | Rich language support, hooks, services |
| `dev-templates` | [nix.dev](https://nix.dev) templates | 43        | Minimal, fast evaluation               |

### Session Model

Sessions are the core unit of state. Each session represents a unique combination of `(provider, languages[])` and is identified by an 8-character hex ID derived from a BLAKE3 hash of its configuration.

```
Session Key Resolution:
  ┌─────────────┐     ┌──────────────┐     ┌────────────────────┐
  │ Index (1,2) │     │ ID (a3f8c2d1)│     │ History (file_path)│
  └──────┬──────┘     └──────┬───────┘     └─────────┬──────────┘
         │                   │                       │
         └───────────────────┼───────────────────────┘
                             ▼
                    ┌────────────────┐
                    │ Session Lookup │
                    └────────┬───────┘
                             ▼
                    ┌────────────────┐
                    │ nix develop    │
                    └────────────────┘
```

Sessions persist across invocations. The `restore` command without arguments presents a history selector for the current working directory.

### Flake Generation

Flakes are generated programmatically from provider templates. The `get_flake_contents` function dispatches to the appropriate provider-specific generator, which composes a `flake.nix` from the selected languages.

## Interface

```
ah <languages...>          Create/enter environment
ah use <languages...>      Alias for above
ah restore [key]           Restore session (shows history if no key)
ah update [session]        Update flake dependencies
ah session list            List all sessions
ah session remove <keys>   Delete sessions
ah session clear           Delete all sessions
ah provider list           List available providers
ah provider show <name>    Show provider language support
ah completion <shell>      Print shell completion script
```

Positional arguments are also supported: `ah rust go python` is equivalent to `ah use rust go python`.

### Configuration

TOML-based config with environment variable override (`AH_*`). Schema validated via `schemars`. Default config is atomically written on first run.

```toml
provider = "devenv"
log = "info"
shell = "zsh"
```

### Shell Completion

Supports bash, zsh, fish, elvish, powershell. Completions for languages and session keys are context-aware.

## License

MIT
