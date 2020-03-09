#!/bin/sh
# Helper script to execute benchmarks on remote (AWS) Linux machines.
set -e
MACHINE="$1"
TARGET="x86_64-unknown-linux-musl"
shift

cargo build --release --package zkp-primefield --target $TARGET  --example fft --all-features
BENCH=(target/$TARGET/release/examples/fft)
scp $BENCH $MACHINE:~/fft
ssh -t $MACHINE "~/fft $(printf '%q ' "$@")"
