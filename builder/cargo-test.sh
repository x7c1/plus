#!/usr/bin/env bash

. $(dirname $0)/setup-env.sh
cd "$PROJECT_ROOT"

cargo build \
  --verbose \
  $(get_build_mode $@) \
  --target=${TARGET_X86}

cargo build \
  --verbose \
  $(get_build_mode $@) \
  --target=${TARGET_ARM}

cargo test \
  $(get_build_mode $@) \
  --target=${TARGET_X86} \
  -- --nocapture
