#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

# enable alias on bash
shopt -s expand_aliases

. ./builder/setup-functions.sh

OPT_LEVEL=$(get_opt_level "$@")
BUILD_MODE=$(get_build_mode "$@")

# defined for this project
export BUILD_MODE
export PROJECT_ROOT="/wasabi"

export OSX_SDK="MacOSX10.15.sdk.tar.bz2"
export OSX_SDK_CC="x86_64-apple-darwin19-clang"
export OSXCROSS_ROOT="${WSB_WORKSPACE}/osxcross"

export TARGET_X86="x86_64-unknown-linux-musl"
export TARGET_ARMV7="armv7-unknown-linux-musleabihf"
export TARGET_MACOS="x86_64-apple-darwin"

export MAX_PARALLEL=$(getconf _NPROCESSORS_ONLN)

# used by rustc
export RUSTFLAGS="-C opt-level=$OPT_LEVEL"

cd ${PROJECT_ROOT}
. ./builder/build-osxcross.sh

if is_osx_sdk_installed; then
  PATH=${OSXCROSS_ROOT}/target/bin:$PATH
fi
