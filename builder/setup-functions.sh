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

get_artifact_names() {
  names=(
    "wsb_pilot_tests"
    "s3api"
  )
  echo ${names[@]}
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

build_apps() {
  cargo build \
    --verbose \
    ${BUILD_MODE} \
    --target=$1
}

build_e2e_tests() {
  cargo test \
    ${BUILD_MODE} \
    --target=$1 \
    --package=wsb-pilot \
    --no-run \
    --message-format=json \
    | jq -r "select(.profile.test == true) | .filenames[]" \
    | grep wsb_pilot_tests
}

copy_release_apps() {
  target_dir=${PROJECT_ROOT}/target/$1/release
  for name in $(get_artifact_names); do
    app=${target_dir}/${name}
    if [[ -f ${app} ]]; then
      cp ${app} ${ARTIFACTS_DIR}/$2/${name}
    fi
  done
}

is_osx_sdk_installed() {
  target=${OSXCROSS_ROOT}/target/bin/${OSX_SDK_CC}
  if [[ -f ${target} ]]; then
    return 0
  else
    return 1
  fi
}

setup_artifacts_directory() {
  [[ -d ${ARTIFACTS_DIR} ]] || mkdir ${ARTIFACTS_DIR}

  for arch in $(get_arch_labels); do
    arch_dir="${ARTIFACTS_DIR}/${arch}"
    [[ -d ${arch_dir} ]] || mkdir ${arch_dir}
  done
}
