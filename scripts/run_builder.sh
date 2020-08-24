#!/usr/bin/env bash

. ./scripts/setup_env.sh

MOUNT_DIR=/plus
task_path="./target/x86_64-unknown-linux-musl/debug/plus-task"
#command=("$@")

main() {
  set -x
  if [ ! -e "${task_path}" ]; then
    write_main "${MOUNT_DIR}/builder/call.sh build_task_runner"
    run
  fi
  write_main "$(quote_args "$@")"
  run
}

run() {
  if [[ "$(current_container)" ]]; then
    restart_container
  else
    run_container
  fi
}

quote_args () {
  for arg in "$@"; do
    printf %s "\"$arg\" "
  done
}

main "$@"
