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
export ARTIFACTS_DIR="${PLUS_PROJECT_ROOT}/dist"

export TARGET_X86="x86_64-unknown-linux-musl"
export TARGET_ARMV7="armv7-unknown-linux-musleabihf"
export TARGET_MACOS="x86_64-apple-darwin"

export ARMV7_CC="arm-linux-gnueabihf-gcc"
export MAX_PARALLEL=$(getconf _NPROCESSORS_ONLN)

# used by rustc
#export RUSTFLAGS="-C opt-level=$OPT_LEVEL"
