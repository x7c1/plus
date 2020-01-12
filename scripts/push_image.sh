#!/usr/bin/env bash

. ./scripts/setup-env.sh

repository="x7c1/wasabi"

docker commit ${CONTAINER_NAME} ${repository}

docker push ${repository}:latest
