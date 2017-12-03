all:
	cargo build --manifest-path ../../../rvs-capi/Cargo.toml
	g++ -std=c++11 test.cpp -L../../../rvs-capi/target/debug -I../include -lrvs -o test
	LD_LIBRARY_PATH=../../../rvs-capi/target/debug ./test
