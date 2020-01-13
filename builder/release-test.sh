#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

alias s3api="$PROJECT_ROOT"/target/x86_64-unknown-linux-musl/release/s3api

s3api put-object \
  --bucket=sample_bucket \
  --key=sample.txt \
  --body=src.txt

s3api get-object \
  --bucket=sample_bucket \
  --key=sample.txt \
  dst.txt
