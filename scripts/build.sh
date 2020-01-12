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

  println "about x86_64 binary"
  show_detail "x86_64-unknown-linux"

  println "about armv7 binary"
  show_detail "armv7"

  println "about macOS binary"
  show_detail "apple"
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
  # todo: use docker
  sudo strip ./target/x86_64-unknown-linux-musl/release/s3api
}

show_detail() {
  set +e # ignore errors

  list=$(list_artifacts | grep $1)
  if [[ ! $? -eq 0 ]]; then
    echo "artifact not found: $1"
    return
  fi

  echo "$list" | xargs file | sed -e "s/,/,\n /g"
  echo "ldd:"
  list_artifacts | grep $1 | xargs ldd
  set -e
}

main
