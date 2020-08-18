#!/usr/bin/env bash

set -xue

export DEBIAN_FRONTEND="noninteractive"
./builder/init/apt-get.sh

. ./builder/init.local/setup_rust.sh

. ./builder/init/init.linux_x86.sh

./builder/init/tools.linux_x86.sh
