#!/usr/bin/env bash

. $(dirname $0)/setup-env.sh
cd "$PROJECT_ROOT"

while getopts ":-:" OPT
do
  case ${OPT} in
    -)  case "${OPTARG}" in
          all-check)
            cargo fmt --verbose --all -- --check
            ;;
          emit-files)
            cargo fmt --verbose -- --emit files
            ;;
          help)
            cargo fmt --help
            rustfmt --help
            rustfmt --help=config
            rustfmt --help=file-lines
            ;;
        esac
        ;;
  esac
done
