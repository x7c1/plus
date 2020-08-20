#!/usr/bin/env bash

# usage:
# $ ./scripts/run_builder.sh task-runner.sh
# $ ./scripts/run_builder.sh task-runner.sh build-apps

. ./builder/setup-env.sh --debug --opt-level=0
. ./builder/lib.linux_x86.sh
cd "$PLUS_PROJECT_ROOT" || exit 1

main() {
  # show executed commands.
  set -x

  cargo_fmt

  task_runner_for_linux_x86 "${@:1}"
}

main "${@}"
