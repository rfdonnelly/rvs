.PHONY: test
test:
	cargo test --all
	(cd c-api/bindings/c_cpp/cpp_example && make)
	rustup toolchain install nightly
	rustup run nightly cargo bench
