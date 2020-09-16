#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

get_opt_level() {
  while getopts ":-:" OPT; do
    case "${OPT}" in
      -)  case "${OPTARG}" in
            opt-level=*)
              echo ${OPTARG#*=}
              exit
              ;;
          esac
          ;;
    esac
  done
  echo 2
}

get_build_mode() {
  while getopts ":-:" OPT; do
    case "${OPT}" in
      -)  case "${OPTARG}" in
            debug)
              echo ""
              exit
              ;;
          esac
          ;;
    esac
  done
  echo "--release"
}

build_apps() {
  cargo build \
    --verbose \
    ${BUILD_MODE} \
    --target=$1
}

build_e2e_tests() {
  build_pilot $1
  build_pilot_and_output_json $1
}

build_pilot() {
  cargo test \
    ${BUILD_MODE} \
    --target=$1 \
    --package=plus-pilot \
    --no-run
}

# call build_pilot in advance to see compilation errors,
# since this function hides them by the --message-format option.
build_pilot_and_output_json() {
  cargo test \
    ${BUILD_MODE} \
    --target=$1 \
    --package=plus-pilot \
    --no-run \
    --message-format=json \
    | jq -r "select(.profile.test == true) | .filenames[]" \
    | grep plus_pilot_tests
}

task_runner() {
  cargo run \
    ${BUILD_MODE} \
    --target=$1 \
    --package=plus-task \
    "${@:2}"
}
