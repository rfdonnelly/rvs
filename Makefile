.PHONY: test
test:
	cargo test --all
	(cd rvs-capi/bindings/c_cpp/cpp_example && make)
