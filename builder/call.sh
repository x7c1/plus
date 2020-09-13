#!/usr/bin/env bash

set -ue

. ./builder/assemble.sh

eval "$(quote_args "$@")"
