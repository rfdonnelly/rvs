all:
	cargo build
	g++ -std=c++11 test.cpp -L../../../target/debug -I../include -lrvs -o test
	LD_LIBRARY_PATH=../../../target/debug ./test
