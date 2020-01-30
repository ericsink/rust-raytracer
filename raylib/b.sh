#!/bin/sh
RUSTFLAGS="--emit=llvm-bc -C panic=abort" cargo build --release

