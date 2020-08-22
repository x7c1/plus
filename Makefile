target=linux_x86

# rf. https://stackoverflow.com/a/10699588
CFLAGS = -c -g -D $(target)

# usage: make assemble target=linux_armv7
assemble:
	./scripts/run_builder.sh assemble.sh $(target)

build-task:
	./scripts/run_builder.sh build-task.sh --debug --opt-level=0

cargo-clippy:
	./scripts/run_builder.sh cargo-clippy.sh --debug --opt-level=0

cargo-fmt:
	./scripts/run_builder.sh cargo-fmt.sh --emit-files

cargo-fmt-check:
	./scripts/run_builder.sh cargo-fmt.sh --all-check

cargo-test:
	./scripts/run_builder.sh cargo-test.sh --debug --opt-level=0

draft:
	./scripts/run_builder.sh draft.local.sh --debug --opt-level=0
