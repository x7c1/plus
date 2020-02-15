#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

# enable alias on bash
shopt -s expand_aliases

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

build_for_x86_linux() {
  cargo build \
    --verbose \
    ${BUILD_MODE} \
    --target="${TARGET_X86}"
}

build_for_armv7_linux() {
  CC=arm-linux-gnueabihf-gcc \
  PKG_CONFIG_ALLOW_CROSS=1 \
  cargo build \
    --verbose \
    ${BUILD_MODE} \
    --target="${TARGET_ARMV7}"
}

build_for_macos() {
  if is_osx_sdk_installed; then
    CC=${OSX_SDK_CC} \
    cargo build \
      --verbose \
      ${BUILD_MODE} \
      --target="${TARGET_MACOS}"
  fi
}
