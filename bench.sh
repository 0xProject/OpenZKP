#!/bin/sh
# Helper script to execute benchmarks on remote (AWS) Linux machines.
set -e
MACHINE="$1"
TARGET="x86_64-unknown-linux-musl"
shift

rm target/$TARGET/release/benchmark-* || true
cargo build --release --package zkp-primefield --target $TARGET  --bench benchmark --all-features
BENCH=(target/$TARGET/release/benchmark-*)
scp $BENCH $MACHINE:~/benchmark
printf -v ARGS %q "$@"
echo "~/benchmark --color --bench $ARGS"
ssh -t $MACHINE "~/benchmark --color --bench $ARGS"
