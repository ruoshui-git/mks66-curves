all:
	cargo run --release

clean:
	cargo clean
	rm -f *.png *.ppm *.gif