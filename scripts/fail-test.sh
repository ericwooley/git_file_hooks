#!/usr/bin/env bash
cargo build --release;
cp ./target/release/git_file_hooks tmp/update;
strip tmp/update;
if ./tmp/update HEAD HEAD~3 update; then
    echo "Success, oh no, this should fail";
    exit 1;
fi
echo "failure, as expected"
exit 0

