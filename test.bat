@echo off

cargo build
pushd gen-test
..\target\debug\java-oxide.exe generate --config java-oxide.toml
popd
