#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

assemble() {
  set +x
  build_target=$1

  task build-apps \
    --target="$build_target" --release

  task setup-artifacts-directory \
    --target="$build_target" --release

  task assemble-pilot-tests \
    --target="$build_target" --release

  task copy-artifact-files \
    --target="$build_target" --release

  # todo:
  exit

  task package-artifacts \
    --target="$build_target" --release

  ls -lh dist/"$build_target"

  task strip-executables \
    --target="$build_target" --release

  task package-artifacts \
    --target="$build_target" --release

  ls -lh dist/"$build_target"

  echo "done."
}

assemble_debug_pilot() {
  target="x86_64-unknown-linux-musl"
  task setup-artifacts-directory --target="$target"
  task assemble-pilot-tests --target="$target"
  task copy-artifact-files --target="$target"
}

build_task_runner() {
  export RUSTFLAGS="-C opt-level=0"
  cargo build --target=x86_64-unknown-linux-musl \
    --package=plus-task

  strip target/x86_64-unknown-linux-musl/debug/plus-task
}

cargo_clippy() {
  export RUSTFLAGS="-C opt-level=0"
  cargo clippy --target=x86_64-unknown-linux-musl
}

cargo_fmt() {
  export RUSTFLAGS="-C opt-level=0"
  cargo fmt --verbose -- --emit files
}

cargo_fmt_check() {
  export RUSTFLAGS="-C opt-level=0"
  cargo fmt --verbose --all -- --check
}

cargo_test() {
  export RUSTFLAGS="-C opt-level=0"
  cargo test --target=x86_64-unknown-linux-musl \
    --workspace --exclude=plus-pilot \
    -- --nocapture
}

task() {
  if [ -e ./plus-task ]; then
    # when called by GitHub Actions using actions/download-artifact
    plus_task_path="./plus-task"
    chmod u+x "$plus_task_path"
  elif [ -e ./target/x86_64-unknown-linux-musl/debug/plus-task ]; then
    # when called by local Docker container
    plus_task_path="./target/x86_64-unknown-linux-musl/debug/plus-task"
  else
    echo "plus-task not found"
    exit 1
  fi
  line="$plus_task_path $(quote_args "$@")"
  eval "$line"
}

quote_args () {
  for arg in "$@"; do
    printf %s "'$arg' "
  done
}
