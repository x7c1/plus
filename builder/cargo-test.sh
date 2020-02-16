#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

run_unit_tests

# usage:
# $ ./scripts/run_tests.sh
# or
# $ ./scripts/run_builder.sh cargo-test.sh --debug --opt-level=0
