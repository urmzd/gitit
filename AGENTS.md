# AGENTS.md

## Identity

You are an agent working on **gitit** — an AI-powered git command tool that generates atomic, conventional commits using Claude or Gemini. Also handles code review, branch naming, PR generation, and repo Q&A.

## Architecture

Single Rust binary with async runtime.

| File | Role |
|------|------|
| `src/main.rs` | CLI entry point (`clap` with derive + env) |

### Subcommands

| Command | Description |
|---------|-------------|
| `commit` | Analyze changes, plan atomic conventional commits |
| `review` | AI-powered code review |
| `branch` | Suggest conventional branch names |
| `pr` | Generate PR title and body |
| `explain` | Explain recent commits |
| `ask` | Freeform repo Q&A |
| `cache status` | Show cached entries |
| `cache clear` | Clear cache |

### Two-Tier Cache System

1. **Exact match** — SHA-256 of sorted file fingerprints + flags → instant result
2. **Incremental** — >50% files unchanged → previous plan as hints to AI

Cache location: `~/.cache/gitit/<repo-id>/entries/` (XDG-compliant). Entries expire after 24h, LRU cap of 20 per repo.

### AI Backends

- **Claude CLI** (`claude`) — uses budget flag
- **Gemini CLI** (`gemini`) — auto-detected if installed

## Key Files

- `src/main.rs` — CLI entry point and subcommand routing
- `Cargo.toml` — Dependencies (clap, tokio, sha2, dirs, crossterm, indicatif)

## Commands

| Task | Command |
|------|---------|
| Build | `just build` or `cargo build --release` |
| Test | `just test` or `cargo test` |
| Lint | `just lint` or `just clippy` + `just fmt-check` |
| Format | `just fmt` or `cargo fmt` |
| Install | `just install` or `cargo install --path .` |
| Full CI | `just ci` (lint + test) |

## Code Style

- Rust 2024 edition, Apache-2.0 license
- `cargo fmt` and `cargo clippy -- -D warnings`
- Async with `tokio`
- `clap` derive with env var support (`GITIT_BACKEND`, `GITIT_MODEL`, `GITIT_BUDGET`)
- TUI progress via `indicatif` and `crossterm`

## Configuration

| Flag | Env Var | Description |
|------|---------|-------------|
| `--backend` | `GITIT_BACKEND` | `claude` or `gemini` (auto-detected) |
| `--model` | `GITIT_MODEL` | Model name (e.g., `haiku`, `sonnet`) |
| `--budget` | `GITIT_BUDGET` | Max USD budget, Claude only (default: 0.50) |
| `--debug` | `GITIT_DEBUG` | Enable debug output |
