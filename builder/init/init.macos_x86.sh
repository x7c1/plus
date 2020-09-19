#!/usr/bin/env bash

set -xue

main () {
  rustup target add --toolchain "${PLUS_RUST_VERSION}" x86_64-apple-darwin

  if is_osxcross_installed; then
    printf "[skip] already installed: osxcross\n\n"
  else
    git_clone_osxcross
    move_sdk_tarball
    install_osxcross
  fi
}

git_clone_osxcross() {
  if [ -d "$OSXCROSS_ROOT/tarballs" ]; then
    printf "[skip] already cloned: osxcross\n\n"
  else
    cd "${PLUS_PROJECT_ROOT}/builder"
    git clone https://github.com/tpoechtrager/osxcross.git
  fi
}

move_sdk_tarball() {
  if [ -f "$OSX_SDK_TARBALL_PATH" ]; then
    printf "[skip] already moved: %s\n\n" "$OSX_SDK_TARBALL"
    return
  fi

  target="$PLUS_PROJECT_ROOT/builder/$OSX_SDK_TARBALL"
  if [ ! -f "$target" ]; then
    printf "[failed] not found: %s\n\n" "$target"
    exit 1
  fi

  mv "$target" "$OSX_SDK_TARBALL_PATH"
}

install_osxcross() {
  cd "$OSXCROSS_ROOT" || exit 1
  UNATTENDED=1 ./build.sh
}

is_osxcross_installed() {
  target=${OSXCROSS_ROOT}/target/bin/${OSX_SDK_CC}
  if [ -f "${target}" ]; then
    return 0
  else
    return 1
  fi
}

main
