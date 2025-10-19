clean:
	cd oneloop && ./clean.sh
	cargo clean

rebuild: clean
	cargo build
