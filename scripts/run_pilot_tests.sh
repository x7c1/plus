#!/usr/bin/env bash

# Usage:
# 1.
#   $ cp ./scripts/setup_pilot_env.local.template ./scripts/setup_pilot_env.local.sh
# 2.
#   $ vim ./scripts/setup_pilot_env.local.sh
# 3.
#   $ ./scripts/run_pilot_tests.sh
#
# See also:
#   https://doc.rust-lang.org/book/ch11-02-running-tests.html


# x: show executed commands.
# u: not allow undefined values.
# e: stop if non-zero returned.
set -xue

./scripts/run_builder.sh assemble_debug_pilot

PLUS_WORKSPACE_DIR="$(pwd)/dist/linux_x86-debug/plus_pilot_workspace"
export PLUS_WORKSPACE_DIR

PLUS_APPS_DIR="$(pwd)/dist/linux_x86-debug"
export PLUS_APPS_DIR

. ./scripts/setup_pilot_env.local.sh

# Print stdout:
./dist/linux_x86-debug/plus_pilot_tests --nocapture

# Disable parallel run:
#./dist/linux_x86-debug/plus_pilot_tests --nocapture --test-threads=1

# Colorize:
#./dist/linux_x86-debug/plus_pilot_tests --nocapture --color always

# Filter target tests
#./dist/linux_x86-debug/plus_pilot_tests get_object --nocapture --color always

exit 1

./scripts/run_builder.sh \
  build-and-test.sh \
    --debug \
    --opt-level=0

output=run_pilot_path.tmp

./scripts/run_builder.sh \
  build-pilot.sh \
    --debug \
    --opt-level=0 \
    --pilot-output=${output}

PLUS_APPS_DIR="$(pwd)/target/x86_64-unknown-linux-musl/debug"
export PLUS_APPS_DIR

PLUS_WORKSPACE_DIR="$(pwd)/dist.bundle/plus_pilot_workspace"
export PLUS_WORKSPACE_DIR

. ./scripts/run_pilot.local.sh

$(cat ./scripts/${output}) "$@"
