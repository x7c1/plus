#!/usr/bin/env bash

set -ue

export OSX_SDK_CC="x86_64-apple-darwin19-clang"
export OSXCROSS_ROOT="${PLUS_PROJECT_ROOT}/builder/osxcross"
export PATH="$OSXCROSS_ROOT"/target/bin:$PATH
export OSX_SDK_TARBALL="MacOSX10.15.sdk.tar.xz"
export OSX_SDK_TARBALL_PATH="$OSXCROSS_ROOT/tarballs/$OSX_SDK_TARBALL"
