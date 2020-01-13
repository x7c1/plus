#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

main() {
  build_for_release

  println "before strip"
  show_file_size

  println "stripping..."
  strip_files

  println "after strip"
  show_file_size

  println "about linux (x86_64) binary"
  show_detail "x86_64-unknown-linux"

  println "about linux (armv7) binary"
  show_detail "armv7"

  println "about macOS (x86_64) binary"
  show_detail "apple"

  println "done."
}

println() {
  echo -e "\n>> $1"
}

build_for_release() {
  # rf.
  # [What do the optimization levels `-Os` and `-Oz` do in rustc? - Stack Overflow]
  # https://stackoverflow.com/questions/45608392/what-do-the-optimization-levels-os-and-oz-do-in-rustc
  ./scripts/run_builder.sh \
    cargo-build.sh --release --opt-level=z
}

list_artifacts() {
  find ./ -type f -name "s3api" \
    | grep release
}

show_file_size() {
  list_artifacts | xargs ls -lh
}

strip_files() {
  ./scripts/run_builder.sh strip-files.sh
}

show_detail() {
  set +e # ignore errors

  list=$(list_artifacts | grep $1)
  if [[ ! $? -eq 0 ]]; then
    echo "artifact not found: $1"
    return
  fi

  # rf.
  # [How to add new line using sed on MacOS? - Stack Overflow]
  # https://stackoverflow.com/questions/16576197/how-to-add-new-line-using-sed-on-macos
  echo "$list" | xargs file | sed $'s/,/,\\\n /g'
  set -e
}

main
