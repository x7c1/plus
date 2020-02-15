#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

# show executed commands.
set -x

build_for_x86_linux

build_for_armv7_linux

build_for_macos
