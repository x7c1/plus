#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PLUS_PROJECT_ROOT" || exit 1

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

# usage:
# $ ./scripts/run_builder.sh cargo-fmt.sh --emit-files
# $ ./scripts/run_builder.sh cargo-fmt.sh --all-check
