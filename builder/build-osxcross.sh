#!/usr/bin/env bash

target_sdk_path="$PROJECT_ROOT/builder/$OSX_SDK"
if [[ ! -f "$target_sdk_path" ]]; then
  echo "osx sdk not found. [$target_sdk_path]"
  exit
fi

tarballs_path="$OSXCROSS_ROOT/tarballs/$OSX_SDK"
if [[ -f "$tarballs_path" ]]; then
  echo "already copied [$tarballs_path]"
else
  cp ${target_sdk_path} ${tarballs_path}
fi

if has_osx_sdk; then
  echo "osx sdk already installed."
  exit
fi

cd "$OSXCROSS_ROOT" || exit 1
UNATTENDED=1 ./build.sh
