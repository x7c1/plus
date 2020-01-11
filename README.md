# wasabi

S3 Client for Rust

## Requirements

Install `docker` and `docker-compose` first.

## Setup

Create a container to build crates:

```
$ ./scripts/setup-builder-docker.sh
```

## Build

Build crates on the container:

```
$ ./scripts/run_tests.sh
```
