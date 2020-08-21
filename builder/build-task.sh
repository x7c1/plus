#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PLUS_PROJECT_ROOT" || exit 1

# shellcheck disable=SC2086
cargo build \
  ${BUILD_MODE} \
  --target="$TARGET_X86" \
  --package=plus-task

# usage:
# $	./scripts/run_builder.sh build-task.sh --debug --opt-level=0
