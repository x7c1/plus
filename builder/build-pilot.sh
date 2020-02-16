#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

. ./builder/lib.linux_x86.sh

main() {
  pilot_output_path=$(get_pilot_output_path $@)

  # show executed commands.
  set -x

  cargo_fmt

  build_apps_for_linux_x86

  run_unit_tests_for_linux_x86

  build_e2e_tests_for_linux_x86 \
    | sed s/^\\${PROJECT_ROOT}/\\./ \
    > ${PROJECT_ROOT}/scripts/${pilot_output_path}
}

get_pilot_output_path() {
  while getopts ":-:" OPT; do
    case "${OPT}" in
      -)  case "${OPTARG}" in
            pilot-output=*)
              echo ${OPTARG#*=}
              exit
              ;;
          esac
          ;;
    esac
  done
  echo "--pilot-output required."
  exit 1
}

main $@
