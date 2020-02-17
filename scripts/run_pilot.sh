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

output=run_pilot_path.tmp

./scripts/run_builder.sh \
  build-pilot.sh \
    --debug \
    --opt-level=0 \
    --pilot-output=${output}

export WSB_APPS_DIR="./target/x86_64-unknown-linux-musl/debug"

. ./scripts/run_pilot.local.sh

$(cat ./scripts/${output}) $@
