#!/usr/bin/env bash

set -euxo pipefail

rm -f src/chips/*/*.rs

(cd ../chiptool/; cargo build)
export RUST_BACKTRACE=1
#export RUST_LOG=info
chiptool=../chiptool/target/debug/chiptool

for chip in $(ls svd); do 
    chip=${chip%.*}
    $chiptool generate --svd svd/$chip.svd --transform transform.yaml
    rustfmt lib.rs
    sed -i '/#!\[no_std]/d' lib.rs

    mkdir -p src/chips/$chip
    mv lib.rs src/chips/$chip/pac.rs
done

cargo fmt
for chip in $(ls svd); do 
    chip=${chip%.*}
    cargo check --features $chip
    cargo doc --features $chip
done
