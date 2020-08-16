#!/usr/bin/env bash

. ./builder/setup-env.sh
. ./builder/lib.linux_x86.sh
cd "$PLUS_PROJECT_ROOT" || exit 1

main() {
  pilot_output_path=$(get_pilot_output_path $@)

  # show executed commands.
  set -x

  build_e2e_tests_for_linux_x86 \
    | sed s/^\\${PLUS_PROJECT_ROOT}/\\./ \
    > ${PLUS_PROJECT_ROOT}/scripts/${pilot_output_path}
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
