
assemble:
	./scripts/run_builder.sh assemble.sh

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
