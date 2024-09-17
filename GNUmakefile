build:
	cargo build

release:
	cargo build --release

clean:
	rm -rf ./target/

rebuild: clean build

format:
	cargo +nightly fmt

.PHONY: build clean format rebuild release
