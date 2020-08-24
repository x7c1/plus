#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

publish_libraries() {
  set +x

  files=$1
  echo "$files"

  current=$(git branch --show-current)
  echo "current branch...$current"

  git diff master..."${current}" --name-only --diff-filter=AM | cat

  cargo publish --dry-run --manifest-path ./libs/env-extractor/Cargo.toml

  cargo publish --dry-run --manifest-path ./libs/aws/plus-s3/Cargo.toml

  echo "done."
}

quote_args () {
  for arg in "$@"; do
    printf %s "\"$arg\" "
  done
}
