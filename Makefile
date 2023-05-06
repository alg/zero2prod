.PHONY: test
test:
	ulimit -n 10000
	cargo test

.PHONY: migrate
migrate:
	SKIP_DOCKER=1 ./scripts/init_db.sh
