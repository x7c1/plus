#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

"$PROJECT_ROOT"/target/x86_64-unknown-linux-musl/release/s3api \
  put-object \
    --bucket=hoge \
    --key=fuga

"$PROJECT_ROOT"/target/x86_64-unknown-linux-musl/release/s3api \
  get-object \
    --bucket=hoge \
    --key=fuga \
    piyo
