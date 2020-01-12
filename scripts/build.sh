#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

# show executed commands.
set -x

main() {
  build_for_release

  println "before strip"
  show_file_size

  println "stripping..."
  strip_files

  println "after strip"
  show_file_size
}

println() {
  set +x
  echo -e "\n>> $1"
  set -x
}

build_for_release() {
  # rf.
  # [What do the optimization levels `-Os` and `-Oz` do in rustc? - Stack Overflow]
  # https://stackoverflow.com/questions/45608392/what-do-the-optimization-levels-os-and-oz-do-in-rustc
  ./scripts/run_builder_task.sh \
    cargo-build --release --opt-level=z
}

show_file_size() {
  find ./ -type f -name "s3api" \
    | grep release \
    | xargs ls -lh
}

strip_files() {
  sudo strip ./target/x86_64-unknown-linux-musl/release/s3api
}

main
