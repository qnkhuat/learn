default: build

setup:
	# add non OS-dependent target
	rustup target add thumbv7em-none-eabihf
	# use nightly features
	rustup override set nightly
	# add src code so we can recompile std and core for our own target
	rustup component add rust-src
	# a tool to preppand bootloader to our program
	cargo install bootimage
	# to use bootimage 
	rustup component add llvm-tools-preview

build:
	cargo build

run: 
	cargo run

test:
	cargo test
