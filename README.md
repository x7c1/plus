# wasabi

S3 Client for Rust

## Requirements

Install `docker` first.

## Setup

Create a container image to build crates:

```
$ ./scripts/setup_builder_docker.sh
```

## Build

Build crates on the container:

```
$ ./scripts/build.sh
```

### For macOS

Pre-installed bash is likely too old.

```
$ bash --version
GNU bash, version 3.2.57(1)-release (x86_64-apple-darwin19)
Copyright (C) 2007 Free Software Foundation, Inc.
```

Upgrade bash if this makes build script failed.

```
$ brew update
$ brew install bash

$ bash --version
GNU bash, version 5.0.11(1)-release (x86_64-apple-darwin19.0.0)
Copyright (C) 2019 Free Software Foundation, Inc.
```
