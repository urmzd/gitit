---
name: gitit
description: AI-powered git commands — generate atomic conventional commits, code reviews, branch names, and PR descriptions using Claude or Gemini. Use when committing, reviewing code, or creating PRs with AI assistance.
argument-hint: [subcommand]
---

# Gitit — AI Git Commands

Use `gitit` for AI-powered git operations.

## Common Usage

```sh
# Generate atomic conventional commits
gitit commit

# Staged changes only
gitit commit --staged

# Dry run (preview without executing)
gitit commit --dry-run

# Skip confirmation
gitit commit --yes

# AI code review
gitit review

# Suggest branch name
gitit branch

# Generate PR description
gitit pr

# Explain recent commits
gitit explain

# Ask about the repo
gitit ask "how does authentication work?"
```

## Cache

```sh
gitit cache status    # Show cached entries
gitit cache clear     # Clear cache for this repo
gitit cache clear --all  # Clear all caches
```

## Configuration

Set backend via `--backend` flag or `GITIT_BACKEND` env var (`claude` or `gemini`). Budget via `--budget` or `GITIT_BUDGET` (default: $0.50, Claude only).
