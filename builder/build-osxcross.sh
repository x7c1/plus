#!/usr/bin/env bash

if is_osx_sdk_installed; then
  echo "OSX SDK already installed."
  return
fi

src_path="$PLUS_PROJECT_ROOT/builder/$OSX_SDK"
if [[ ! -f "$src_path" ]]; then
  echo "OSX SDK tarball not found. [$src_path]"
  return
fi

dst_path="$OSXCROSS_ROOT/tarballs/$OSX_SDK"
if [[ -f "$dst_path" ]]; then
  echo "OSX SDK tarball already copied. [$dst_path]"
else
  cp ${src_path} ${dst_path}
fi

cd "$OSXCROSS_ROOT" || exit 1
UNATTENDED=1 ./build.sh

cd "$PLUS_PROJECT_ROOT" || exit 1
