.PHONY: test
test:
	cargo test --manifest-path rvs-parser/Cargo.toml
	cargo test
	cargo test --manifest-path rvs-capi/Cargo.toml
	(cd bindings/c_cpp/cpp_example && make)
	cargo test --manifest-path rvs-repl/Cargo.toml

