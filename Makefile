target=x86_64-unknown-linux-musl

# rf. https://stackoverflow.com/a/10699588
CFLAGS = -c -g -D $(target)

# usage:
#   make assemble target=armv7-unknown-linux-musleabihf
#   make assemble target=x86_64-apple-darwin
assemble:
	./scripts/run_builder.sh assemble $(target)

build-task:
	./scripts/run_builder.sh build_task_runner

cargo-clippy:
	./scripts/run_builder.sh cargo_clippy

cargo-fmt:
	./scripts/run_builder.sh cargo_fmt

cargo-fmt-check:
	./scripts/run_builder.sh cargo_fmt_check

cargo-test:
	./scripts/run_builder.sh cargo_test

download-osxcross:
	./scripts/download_osxcross.sh

draft:
	[ -e ./builder/draft.local.sh ] || \
		cp ./builder/draft.sample.sh ./builder/draft.local.sh
	./scripts/run_builder.sh ./builder/draft.local.sh

run-pilot-tests:
	./scripts/run_pilot_tests.sh
