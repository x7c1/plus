#!/usr/bin/env bash
#
if is_arm_openssl_installed; then
  echo "OpenSSL for ARM already installed."
  return
fi

cd ${ARM_OPENSSL_ROOT}

./Configure \
  no-asm \
  -static \
  --prefix=${WSB_WORKSPACE_ARM}/local \
  --openssldir=${WSB_WORKSPACE_ARM}/local/openssl \
  -D_FORTIFY_SOURCE=0 \
  linux-armv4

echo "using $MAX_PARALLEL processors."
make \
  CC=arm-linux-gnueabihf-gcc \
  -j ${MAX_PARALLEL}

make install_sw
