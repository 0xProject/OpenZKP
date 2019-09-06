# Rust build environment Dockerfile.
# NOTE: When making changes, also bumb the version number in
# the CircleCI config, or existing builds will break.
#
# See also the Rust playground Dockerfile: 
# https://github.com/integer32llc/rust-playground/blob/master/compiler/base/Dockerfile
FROM circleci/rust:1-node
USER root

# The latest nightly
# TODO: Update manually. 
ENV NIGHTLY="nightly-2019-08-15"

# Compile project to load up global cargo caches.
# We also leave the `.git` and `target` folder around as this
# will speedup CI builds. The `checkout` routine will make sure
# we have a fresh source checkout in CI.
COPY --chown=circleci:circleci . /root/project

RUN true \
 # Install Nightly with rustfmt, wasm and Cortex-M3 support
 && rustup toolchain install $NIGHTLY \
 && rustup target add wasm32-unknown-unknown --toolchain $NIGHTLY \
 && rustup target add thumbv7m-none-eabi --toolchain $NIGHTLY \
 && rustup component add rustfmt --toolchain $NIGHTLY \
 # Install tools
 && rustup component add clippy \
 && cargo install sccache --no-default-features \
 && cargo install --git https://github.com/alexcrichton/wasm-gc \
 && cargo install twiggy \
 && cargo install cargo-cache \
 && cargo install grcov \
 # For the rocksdb dependency of substrate-node
 && apt-get install clang \
 # For coverage reports
 && apt-get install lcov \
 # Build project
 && cd /root/project \
 && cargo build --all --all-targets --all-features \
 && cargo build --release --all --all-targets --all-features \
 # Download codechecks deps
 && cd /root/project/.circleci/codechecks \
 && yarn \
 # Compress cargo caches
 && cargo cache --autoclean-expensive
