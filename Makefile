
assemble:
	./scripts/run_builder.sh assemble.sh linux_x86

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
