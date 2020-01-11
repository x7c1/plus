#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

# show executed commands.
set -x

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
OPT_LEVEL=$(get_opt_level "$@")

# defined for this project
export PROJECT_ROOT="/wasabi"
export TARGET_X86="x86_64-unknown-linux-musl"
export TARGET_ARM="armv7-unknown-linux-musleabihf"

# used by rustc
export RUSTFLAGS="-C opt-level=$OPT_LEVEL"
