#!/usr/bin/env bash

# DEPRECATED

# Usage:
#   $ ./scripts/run_release_pilot.sh [bucket-for-test]
#
# See ./scripts/setup_aws_env.sh if need to load $AWS_*.

if [ -z $1 ]; then
  echo "USAGE:"
  echo $0 '[my-test-bucket]'
  exit 1
fi

# x: show executed commands.
# u: not allow undefined values.
# e: stop if non-zero returned.
set -xue

test_bucket=$1
tmp_dir="./dist.tmp"

[ -d $tmp_dir ] || mkdir $tmp_dir

tar --verbose \
  --extract --file ./dist/linux_x86.tar.xz \
  --directory $tmp_dir

cd $tmp_dir/linux_x86

./run_pilot_tests.sh "$test_bucket"
