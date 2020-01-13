#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

strip ./target/x86_64-unknown-linux-musl/release/s3api

arm-linux-gnueabihf-strip ./target/armv7-unknown-linux-musleabihf/release/s3api

if is_osx_sdk_installed; then
  x86_64-apple-darwin19-strip ./target/x86_64-apple-darwin/release/s3api
fi
