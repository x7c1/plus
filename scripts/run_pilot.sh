#!/usr/bin/env bash

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
