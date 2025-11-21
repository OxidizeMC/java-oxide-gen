#! /bin/bash
set -euo pipefail

cargo build -p java-oxide-gen
cd gen-test
RUST_BACKTRACE=1 ./../target/debug/java-oxide-gen -v generate
cargo fmt -- --config-path ../rustfmt.toml --config "max_width=200"
cargo check >cargo-check-output.txt 2>&1
cd ..
