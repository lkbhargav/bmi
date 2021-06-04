

build:
	cargo clean
	# Raspberry pi
	cargo build --release --target=armv7-unknown-linux-gnueabihf
	# Windows x86
	cargo build --release --target=x86_64-pc-windows-gnu
	# Linux x86
	cargo build --release --target=x86_64-unknown-linux-gnu
	# Mac x86 (https://wapl.es/rust/2019/02/17/rust-cross-compile-linux-to-macos.html)
	cargo build --release --target=x86_64-apple-darwin