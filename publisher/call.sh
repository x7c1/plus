#!/usr/bin/env bash

set -xue

. ./publisher/functions.sh

eval "$(quote_args "$@")"
