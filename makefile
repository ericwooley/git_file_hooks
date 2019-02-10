OS = ${OSTYPE}
test_run:
	cargo run 19b2ea5c076971433d3a8e13a3f602eaf939380e 397dfdfcb846076d0423f9ab5ce3bae80133b551 post-checkout;
prod_test_run_failure:
	cargo build --release;
	cp ./target/release/git_file_hooks tmp/update;
	strip tmp/update;
	./tmp/update 19b2ea5c076971433d3a8e13a3f602eaf939380e 397dfdfcb846076d0423f9ab5ce3bae80133b551 update || exit 0;

prod_test_run_success:
	cargo build --release;
	cp ./target/release/git_file_hooks tmp/post-checkout;
	strip tmp/post-checkout;
	./tmp/post-checkout 19b2ea5c076971433d3a8e13a3f602eaf939380e 397dfdfcb846076d0423f9ab5ce3bae80133b551 || exit 0;

prod_test_run_head_success:
	cargo build --release;
	cp ./target/release/git_file_hooks tmp/post-checkout;
	strip tmp/post-checkout;
	./tmp/post-checkout || exit 0;

test:
	cargo test;

build_prod:
	./build-prod.sh;

build:
	make build_prod;

build-dev:
	cargo build;

clean:
	cargo clean;
	rm -rf build/*;
	rm -rf tmp/*;