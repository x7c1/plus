# Plus

S3 Client for Rust

## Requirements

Install `docker` first.

## Build

Build crates on the container:

```
$ make assemble
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


<!--

## Notes

running `act` not works because `actions/upload-artifact` fails:

```
[assemble/build-task-runner ]   ❗  ::error::Unable to get ACTIONS_RUNTIME_TOKEN env variable
[assemble/build-task-runner ]   ❌  Failure - Upload plus-task
Error: exit with `FAILURE`: 1
```

## Run `act`

To run tests:

```
$ act --reuse --job cargo-test -P ubuntu-latest=nektos/act-environments-ubuntu:18.04
```

To run all jobs:

```
$ act --reuse -P ubuntu-latest=nektos/act-environments-ubuntu:18.04
```
-->
