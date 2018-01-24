all: tokenize_test

clean:
	rm -f tokenize_test

tokenize_test: rust_build c_build

rust_build:
	cargo build --release

c_build:
	gcc -L"./target/release/" -ltokenize test.c -o tokenize_linked

