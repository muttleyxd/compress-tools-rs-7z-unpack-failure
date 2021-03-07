#!/usr/bin/env bash
set -x
rm two_files.7z 1 2 2>/dev/null
touch 1 2
7z a two_files.7z 1 2
cargo build
cargo run -- ./two_files.7z
