.PHONY: clean rebuild develop

clean:
	cd oneloop && ./clean.sh
	cargo clean

rebuild: clean
	cargo build

develop:
	pip install maturin[patchelf]
	maturin develop --release --features python