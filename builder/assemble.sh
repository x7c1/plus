#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

build_target=$1

main() {
  task build-apps \
    --target="$build_target" --release

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
  set -x
  ./target/x86_64-unknown-linux-musl/debug/plus-task "$@"
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

main
