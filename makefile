OS = ${OSTYPE}
test_run:
	cargo run HEAD HEAD~5 post-checkout;

mk_temp:
	mkdir -p tmp;

test_run_failure: mk_temp
	cargo build --release;
	cp ./target/release/git_file_hooks tmp/update;
	strip tmp/update;
	scripts/fail-test.sh;

# depends on test run failure for building the files
test_skip_hooks: test_run_failure
	SKIP_GIT_HOOKS=1 ./tmp/update HEAD HEAD~5 update;

test_run_success: mk_temp
	cargo build --release;
	cp ./target/release/git_file_hooks tmp/post-checkout;
	strip tmp/post-checkout;
	./tmp/post-checkout HEAD HEAD~5;

test_run_head_success: mk_temp
	cargo build --release;
	cp ./target/release/git_file_hooks tmp/post-checkout;
	strip tmp/post-checkout;
	./tmp/post-checkout;

integration_test: test_run_failure test_run_success test_run_head_success
	echo "\n\n\n\n\n🙌 🙌 Integration test passed 🙌 🙌\n\n\n\n\n"

test: integration_test
	cargo test;
	echo "\n\n\n\n\n🙌 🙌 Tests Passed 🙌 🙌\n\n\n\n\n";

build_prod:
	./build-prod.sh;

build:
	./build-prod.sh;

build-dev:
	cargo build;

clean:
	cargo clean;
	rm -rf build/*;
	rm -rf tmp/*;