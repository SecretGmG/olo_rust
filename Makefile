clean:
	cd oneloop && ./clean.sh
	cargo clean

rebuild: clean
	cargo build

develop:
	maturin develop --release --features python