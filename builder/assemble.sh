#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

main() {
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
