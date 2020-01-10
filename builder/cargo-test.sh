#!/usr/bin/env bash

. $(dirname $0)/setup-env.sh
cd /wasabi

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
