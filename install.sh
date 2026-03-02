#!/bin/sh
# install.sh — Installs the git-ai binary from GitHub releases.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/urmzd/git-ai/main/install.sh | sh
#
# Environment variables:
#   GIT_AI_VERSION     — version to install (e.g. "v0.1.0"); defaults to latest
#   GIT_AI_INSTALL_DIR — installation directory; defaults to $HOME/.local/bin

set -eu

REPO="urmzd/git-ai"

main() {
    os=$(uname -s)
    arch=$(uname -m)

    case "$os" in
        Linux)
            case "$arch" in
                x86_64)  target="x86_64-unknown-linux-musl" ;;
                aarch64) target="aarch64-unknown-linux-musl" ;;
                *)       err "Unsupported Linux architecture: $arch" ;;
            esac
            ;;
        Darwin)
            case "$arch" in
                x86_64)  target="x86_64-apple-darwin" ;;
                arm64)   target="aarch64-apple-darwin" ;;
                *)       err "Unsupported macOS architecture: $arch" ;;
            esac
            ;;
        MINGW*|MSYS*|CYGWIN*|Windows_NT)
            err "Windows is not supported by this installer. Download a binary from https://github.com/$REPO/releases/latest"
            ;;
        *)
            err "Unsupported operating system: $os"
            ;;
    esac

    if [ -n "${GIT_AI_VERSION:-}" ]; then
        tag="$GIT_AI_VERSION"
    else
        tag=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" \
            | sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p')
        if [ -z "$tag" ]; then
            err "Failed to fetch latest release tag"
        fi
    fi

    artifact="git-ai-${target}"
    url="https://github.com/$REPO/releases/download/${tag}/${artifact}"

    install_dir="${GIT_AI_INSTALL_DIR:-$HOME/.local/bin}"
    mkdir -p "$install_dir"

    echo "Downloading git-ai $tag for $target..."
    curl -fsSL "$url" -o "$install_dir/git-ai"
    chmod +x "$install_dir/git-ai"

    echo "Installed git-ai to $install_dir/git-ai"

    case ":$PATH:" in
        *":$install_dir:"*) ;;
        *) add_to_path "$install_dir" ;;
    esac
}

add_to_path() {
    install_dir="$1"

    case "$(basename "$SHELL")" in
        zsh)  profile="$HOME/.zshrc" ;;
        bash)
            if [ -f "$HOME/.bashrc" ]; then
                profile="$HOME/.bashrc"
            else
                profile="$HOME/.profile"
            fi
            ;;
        fish) profile="$HOME/.config/fish/config.fish" ;;
        *)    profile="$HOME/.profile" ;;
    esac

    if [ "$(basename "$SHELL")" = "fish" ]; then
        if ! grep -q "$install_dir" "$profile" 2>/dev/null; then
            mkdir -p "$(dirname "$profile")"
            echo "" >> "$profile"
            echo "# Added by git-ai installer" >> "$profile"
            echo "set -Ux fish_user_paths $install_dir \$fish_user_paths" >> "$profile"
            echo "Added $install_dir to $profile"
            echo "Restart your shell or run: source $profile"
        fi
    elif [ -n "$profile" ] && ! grep -q "$install_dir" "$profile" 2>/dev/null; then
        echo "" >> "$profile"
        echo "# Added by git-ai installer" >> "$profile"
        echo "export PATH=\"$install_dir:\$PATH\"" >> "$profile"
        echo "Added $install_dir to $profile"
        echo "Restart your shell or run: source $profile"
    fi
}

err() {
    echo "Error: $1" >&2
    exit 1
}

main
