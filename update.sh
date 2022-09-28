#!/usr/bin/env bash

set -euxo pipefail

rm -rf src
mkdir src

(cd ../chiptool/; cargo build)
RUST_BACKTRACE=1 RUST_LOG=info ../chiptool/target/debug/chiptool generate --svd svd/nrf52840.svd --transform svd/nrf52840.yaml

# cargo install form
form -i lib.rs -o src
rm lib.rs
#mv lib.rs src

cargo fmt
cargo check
cargo doc