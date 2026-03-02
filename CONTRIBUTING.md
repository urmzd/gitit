# Contributing to git-ai

Thanks for your interest in contributing! Here's how to get started.

## Development setup

```sh
git clone https://github.com/urmzd/git-ai.git
cd git-ai
cargo build
```

## Running checks

Before submitting a PR, ensure everything passes:

```sh
cargo check && cargo clippy -- -D warnings && cargo test
```

## Making changes

1. Fork the repo and create a branch from `main`
2. Make your changes
3. Add tests if applicable
4. Run the checks above
5. Open a pull request

## Commit style

This project follows [Angular Conventional Commits](https://www.conventionalcommits.org/). Use `git-ai commit` itself to generate your commit messages if you'd like.

## Reporting issues

Open an issue at <https://github.com/urmzd/git-ai/issues> with:

- What you expected
- What happened instead
- Steps to reproduce
- OS, Rust version, and backend CLI version

## License

By contributing, you agree that your contributions will be licensed under the Apache-2.0 license.
