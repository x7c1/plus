target=linux_x86

# rf. https://stackoverflow.com/a/10699588
CFLAGS = -c -g -D $(target)

# usage: make assemble target=linux_armv7
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

draft:
	[ ! -e ./builder/draft.local.sh ] || \
		cp ./builder/draft.sample.sh ./builder/draft.local.sh
	./scripts/run_builder.sh ./builder/draft.local.sh
