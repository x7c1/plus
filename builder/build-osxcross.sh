#!/usr/bin/env bash

src_path="$PROJECT_ROOT/builder/$OSX_SDK"
if [[ ! -f "$src_path" ]]; then
  echo "osx sdk tarball not found. [$src_path]"
  return
fi

dst_path="$OSXCROSS_ROOT/tarballs/$OSX_SDK"
if [[ -f "$dst_path" ]]; then
  echo "osx sdk tarball already copied. [$dst_path]"
else
  cp ${src_path} ${dst_path}
fi

if is_osx_sdk_installed; then
  echo "osx sdk already installed."
  return
fi

cd "$OSXCROSS_ROOT" || exit 1
UNATTENDED=1 ./build.sh
