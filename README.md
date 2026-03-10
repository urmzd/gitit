# gitit

AI-powered git commands for the lazy. Analyzes your changes and generates atomic, conventional commits automatically.

## Features

- **Smart commit planning** - Groups changes into atomic commits following [Angular Conventional Commits](https://www.conventionalcommits.org/)
- **Multi-backend** - Works with Claude CLI or Gemini CLI
- **Two-tier cache** - Exact-match and incremental caching avoids redundant AI calls
- **Code review** - AI-powered review of staged or branch changes
- **Branch naming** - Suggests conventional branch names
- **PR generation** - Generates PR title and body from branch commits
- **Repo Q&A** - Freeform questions about your codebase

## Installation

### Quick install (Linux/macOS)

```sh
curl -fsSL https://raw.githubusercontent.com/urmzd/gitit/main/install.sh | sh
```

You can pin a version or change the install directory:

```sh
GITIT_VERSION=v0.1.0 GITIT_INSTALL_DIR=/usr/local/bin \
  curl -fsSL https://raw.githubusercontent.com/urmzd/gitit/main/install.sh | sh
```

### From crates.io

```sh
cargo install gitit
```

### From source

```sh
git clone https://github.com/urmzd/gitit.git
cd gitit
cargo install --path .
```

## Prerequisites

At least one AI backend CLI must be installed:

- [Claude CLI](https://docs.anthropic.com/en/docs/claude-code) (`claude`)
- [Gemini CLI](https://github.com/google-gemini/gemini-cli) (`gemini`)

## Usage

### Generate commits

```sh
# Analyze all changes and create atomic commits
gitit commit

# Only analyze staged changes
gitit commit --staged

# Dry run (preview plan without executing)
gitit commit --dry-run

# Skip confirmation prompt
gitit commit --yes

# Add context for the AI
gitit commit --message "focus on the auth refactor"

# Bypass cache (always call AI)
gitit commit --no-cache
```

### Cache management

The cache stores commit plans keyed on file content fingerprints. Identical changes produce instant results without an AI call. When only some files change, the incremental tier provides hints to the AI for faster, more consistent plans.

```sh
# Show cached entries for this repo
gitit cache status

# Clear cache for this repo
gitit cache clear

# Clear cache for all repos
gitit cache clear --all
```

Cache location: `~/.cache/gitit/<repo-id>/entries/` (XDG-compliant). Entries expire after 24 hours with an LRU cap of 20 per repo.

### Other commands

```sh
# AI code review
gitit review

# Suggest branch name
gitit branch

# Generate PR description
gitit pr

# Explain recent commits
gitit explain

# Ask questions about the repo
gitit ask "how does authentication work?"
```

## Configuration

All options can be set via CLI flags or environment variables:

| Flag | Env var | Description |
|------|---------|-------------|
| `--backend` | `GITIT_BACKEND` | `claude` or `gemini` (auto-detected if unset) |
| `--model` | `GITIT_MODEL` | Model name (e.g. `haiku`, `sonnet`) |
| `--budget` | `GITIT_BUDGET` | Max budget in USD, Claude only (default: `0.50`) |
| `--debug` | `GITIT_DEBUG` | Enable debug output |

## How caching works

1. **Fingerprinting** - SHA-256 hash of each file's diff (tracked) or content (untracked)
2. **State key** - SHA-256 of sorted fingerprints + staged flag + user message + backend + model
3. **Tier 1 (exact match)** - If the state key matches a cached entry, return the stored plan instantly
4. **Tier 2 (incremental)** - If no exact match but a recent entry shares >50% of files unchanged, inject the previous plan as hints for the AI

## Agent Skill

This project ships an [Agent Skill](https://github.com/vercel-labs/skills) for use with Claude Code, Cursor, and other compatible agents.

**Install:**

```sh
npx skills add urmzd/gitit
```

Once installed, use `/gitit` to generate commits, review code, create branch names, and draft PRs with AI assistance.

## License

Apache-2.0. See [LICENSE](LICENSE).
