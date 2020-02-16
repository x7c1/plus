#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

get_arch_labels() {
  labels=(
    "linux_x86"
    "linux_armv7"
    "macos_x86"
  )
  echo ${labels[@]}
}

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

is_osx_sdk_installed() {
  target=${OSXCROSS_ROOT}/target/bin/${OSX_SDK_CC}
  if [[ -f ${target} ]]; then
    return 0
  else
    return 1
  fi
}
