.PHONY: test
test:
	ulimit -n 10000
	cargo test
