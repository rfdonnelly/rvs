.PHONY: test
test:
	cargo test --all
	(cd bindings/c_cpp/cpp_example && make)
