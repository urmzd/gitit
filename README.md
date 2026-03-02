# git-ai

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
curl -fsSL https://raw.githubusercontent.com/urmzd/git-ai/main/install.sh | sh
```

You can pin a version or change the install directory:

```sh
GIT_AI_VERSION=v0.1.0 GIT_AI_INSTALL_DIR=/usr/local/bin \
  curl -fsSL https://raw.githubusercontent.com/urmzd/git-ai/main/install.sh | sh
```

### From crates.io

```sh
cargo install git-ai
```

### From source

```sh
git clone https://github.com/urmzd/git-ai.git
cd git-ai
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
git-ai commit

# Only analyze staged changes
git-ai commit --staged

# Dry run (preview plan without executing)
git-ai commit --dry-run

# Skip confirmation prompt
git-ai commit --yes

# Add context for the AI
git-ai commit --message "focus on the auth refactor"

# Bypass cache (always call AI)
git-ai commit --no-cache
```

### Cache management

The cache stores commit plans keyed on file content fingerprints. Identical changes produce instant results without an AI call. When only some files change, the incremental tier provides hints to the AI for faster, more consistent plans.

```sh
# Show cached entries for this repo
git-ai cache status

# Clear cache for this repo
git-ai cache clear

# Clear cache for all repos
git-ai cache clear --all
```

Cache location: `~/.cache/git-ai/<repo-id>/entries/` (XDG-compliant). Entries expire after 24 hours with an LRU cap of 20 per repo.

### Other commands

```sh
# AI code review
git-ai review

# Suggest branch name
git-ai branch

# Generate PR description
git-ai pr

# Explain recent commits
git-ai explain

# Ask questions about the repo
git-ai ask "how does authentication work?"
```

## Configuration

All options can be set via CLI flags or environment variables:

| Flag | Env var | Description |
|------|---------|-------------|
| `--backend` | `GIT_AI_BACKEND` | `claude` or `gemini` (auto-detected if unset) |
| `--model` | `GIT_AI_MODEL` | Model name (e.g. `haiku`, `sonnet`) |
| `--budget` | `GIT_AI_BUDGET` | Max budget in USD, Claude only (default: `0.50`) |
| `--debug` | `GIT_AI_DEBUG` | Enable debug output |

## How caching works

1. **Fingerprinting** - SHA-256 hash of each file's diff (tracked) or content (untracked)
2. **State key** - SHA-256 of sorted fingerprints + staged flag + user message + backend + model
3. **Tier 1 (exact match)** - If the state key matches a cached entry, return the stored plan instantly
4. **Tier 2 (incremental)** - If no exact match but a recent entry shares >50% of files unchanged, inject the previous plan as hints for the AI

## License

Apache-2.0. See [LICENSE](LICENSE).
