#!/usr/bin/env bash

set -ue

main() {
  setup_rust
}

setup_rust() {
  if [ -e "$HOME"/.cargo/env ]; then
    . "$HOME"/.cargo/env
  fi

  if command -v rustc; then
    echo "rust already installed."
    return
  fi
  apt-get install -y curl
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /usr/bin/rustup-init
  chmod +x /usr/bin/rustup-init
  rustup-init -y
  . "$HOME"/.cargo/env
}

main
