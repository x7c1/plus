#!/usr/bin/env bash

build_e2e_tests_for_linux_x86() {
  build_e2e_tests ${TARGET_X86}
}

cargo_fmt() {
  cargo fmt \
    --verbose \
    -- --emit files
}
