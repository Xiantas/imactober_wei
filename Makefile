DL = --features bevy/dynamic_linking
TL = --target=x86_64-unknown-linux-gnu
TW = --target=x86_64-pc-windows-gnu

default: build

build:
	cargo build $(DL)

run:
	cargo run $(DL)

release:
	cargo build --release

clean:
	cargo clean
