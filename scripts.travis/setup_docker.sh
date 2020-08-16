#!/bin/bash

repository="x7c1/wasabi"

docker pull ${repository}:latest

docker tag ${repository}:latest plus_builder
