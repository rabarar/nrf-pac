#!/usr/bin/env bash

set -euxo pipefail


if ! command -v chiptool &> /dev/null; then
    echo "chiptool could not be found. Install it with the following command:"
    echo ""
    echo "    cargo install --git https://github.com/embassy-rs/chiptool --locked"
    echo ""
    exit 1
fi

rm -f src/chips/*/*.rs

export RUST_BACKTRACE=1
#export RUST_LOG=info

for chip in $(ls svd); do 
    chip=${chip%.*}
    chiptool generate --svd svd/$chip.svd --transform transform.yaml
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
