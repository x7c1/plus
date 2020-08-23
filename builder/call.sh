#!/usr/bin/env bash

set -xue

. ./builder/assemble.sh

eval "$*"
