# This Makefile is based off of one I found here: 
# https://www.greyblake.com/blog/2017-08-10-exposing-rust-library-to-c/
GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)

run: clean build
	./examples/c_example
clean:
	$(CARGO_BIN) clean
	rm -f ./examples/c_example
build:
	$(CARGO_BIN) build --release
	$(GCC_BIN) -std=c99 -o ./examples/c_example ./examples/c_example.c -Isrc  -L./target/release/ -lbigbang
