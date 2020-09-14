#!/usr/bin/env bash

set -xue

apt-get update
apt-get install -y \
  clang \
  cmake \
  git \
  libxml2-dev \
  libssl-dev \
  libz-dev \
  patch

#  libz-dev
#   \
#  libssl-dev

#   \
#  clang \
#  cmake

#   \
#  git \
#  libxml2-dev \
#  libssl-dev \
#  libz-dev \
#  patch
