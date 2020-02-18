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
#   $ ./scripts/run_pilot.sh --nocapture


# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

# show executed commands.
set -x

./scripts/run_builder.sh \
  build-and-test.linux_x86.sh \
    --debug \
    --opt-level=0

output=run_pilot_path.tmp

./scripts/run_builder.sh \
  build-pilot.linux_x86.sh \
    --debug \
    --opt-level=0 \
    --pilot-output=${output}

export WSB_APPS_DIR="$(pwd)/target/x86_64-unknown-linux-musl/debug"
export WSB_WORKSPACE_DIR="$(pwd)/wsb-pilot-workspace"

. ./scripts/run_pilot.local.sh

$(cat ./scripts/${output}) $@
