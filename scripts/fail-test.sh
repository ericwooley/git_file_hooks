#!/usr/bin/env bash
cargo build --release;
cp ./target/release/git_file_hooks tmp/update;
strip tmp/update;
if ./tmp/update 19b2ea5c076971433d3a8e13a3f602eaf939380e 397dfdfcb846076d0423f9ab5ce3bae80133b551 update; then
    echo "Success, oh no, this should fail";
    exit 1;
fi
echo "failure, as expected"
exit 0

