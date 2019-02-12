#!/usr/bin/env bash
OS=$OSTYPE;
if [[ "$OSTYPE" == "darwin"* ]]; then
  OS="darwin";
fi

cargo build --release;
strip target/release/git_file_hooks;    
mkdir -p build;
cp target/release/git_file_hooks build/git_file_hooks-$OS;