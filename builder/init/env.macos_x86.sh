#!/usr/bin/env bash

set -ue

export OSX_SDK_CC="x86_64-apple-darwin19-clang"
export OSXCROSS_ROOT="${PLUS_PROJECT_ROOT}/osxcross"
export PATH="$OSXCROSS_ROOT"/target/bin:$PATH
