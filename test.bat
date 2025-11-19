@echo off

cargo build -p java-oxide-gen
pushd gen-test
set "RUST_BACKTRACE=1"
..\target\debug\java-oxide-gen.exe -vv generate
cargo fmt -- --config-path ..\rustfmt.toml --config "max_width=150"
cargo check >cargo-check-output.txt 2>&1
set "RUST_BACKTRACE="
popd
