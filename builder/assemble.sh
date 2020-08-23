#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

if [ -e ./plus-task ]; then
  plus_task_path="./plus-task"
  chmod u+x "$plus_task_path"
elif [ -e ./target/x86_64-unknown-linux-musl/debug/plus-task ]; then
  plus_task_path="./target/x86_64-unknown-linux-musl/debug/plus-task"
else
  echo "plus-task not found"
  exit 1
fi

assemble() {
  set +x
  build_target=$1

  task build-apps \
    --target="$build_target" --release

  task setup-artifacts-directory \
    --target="$build_target"

  task assemble-pilot-tests \
    --target="$build_target" --release

  task copy-artifact-files \
    --target="$build_target" --release

  task package-artifacts \
    --target="$build_target"

  ls -lh dist/"$build_target"

  task strip-executables \
    --target="$build_target"

  task package-artifacts \
    --target="$build_target"

  ls -lh dist/"$build_target"

  println "done."
}

build_task_runner() {
  export RUSTFLAGS="-C opt-level=0"
  cargo build --target=x86_64-unknown-linux-musl \
    --package=plus-task
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

main_old() {
  build_for_release

  println "copying apps..."
  ./builder/copy-as-artifacts.sh

  println "archiving..."
  ./builder/archive-all.sh

  println "before strip"
  show_file_size

  println "stripping..."
  ./builder/strip-files.sh

  println "archiving..."
  ./builder/archive-all.sh

  println "after strip"
  show_file_size

  println "artifact details"
  ./builder/show-artifacts.sh

  println "done."
}

println() {
  echo -e "\n>> $1"
}

task() {
  line="$plus_task_path $(quote_args "$@")"
  eval "$line"
}

quote_args () {
    for arg in "$@"; do
        printf %s "\"$arg\" "
    done
}

build_for_release() {
  # rf.
  # [What do the optimization levels `-Os` and `-Oz` do in rustc? - Stack Overflow]
  # https://stackoverflow.com/questions/45608392/what-do-the-optimization-levels-os-and-oz-do-in-rustc
  ./builder/build-all.sh --release --opt-level=z
}

show_file_size() {
  ls -lh dist/**
}
