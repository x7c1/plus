#!/usr/bin/env bash

# Usage:
# 1.
#   $ cp ./scripts/run_pilot.local.template ./scripts/run_pilot.local.sh
# 2.
#   $ vim ./scripts/run_pilot.local.sh
# 3.
#   $ ./scripts/run_pilot.sh
#
# Enable to print stdout if necessary:
#   $ ./scripts/run_pilot.sh --color always --nocapture
#
# Disable to run in parallel:
#   $ ./scripts/run_pilot.sh --color always --test-threads=1
#
# See also:
#   https://doc.rust-lang.org/book/ch11-02-running-tests.html


# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

# show executed commands.
set -x

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

WSB_APPS_DIR="$(pwd)/target/x86_64-unknown-linux-musl/debug"
export WSB_APPS_DIR

WSB_WORKSPACE_DIR="$(pwd)/dist.bundle/wsb-pilot-workspace"
export WSB_WORKSPACE_DIR

. ./scripts/run_pilot.local.sh

$(cat ./scripts/${output}) "$@"
