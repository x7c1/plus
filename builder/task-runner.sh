#!/usr/bin/env bash

# usage:
# $ ./scripts/run_builder.sh task-runner.sh --debug
# $ ./scripts/run_builder.sh task-runner.sh --debug build-apps

. ./builder/setup-env.sh
. ./builder/lib.linux_x86.sh
cd "$PROJECT_ROOT" || exit 1

main() {
  # show executed commands.
  set -x

  cargo_fmt

  task_runner_for_linux_x86 "${@:2}"
}

main "${@:1}"
