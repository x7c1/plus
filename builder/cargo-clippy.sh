#!/usr/bin/env bash

. ./builder/setup-env.sh
. ./builder/lib.linux_x86.sh
cd "$PLUS_PROJECT_ROOT" || exit 1

cargo_clippy
