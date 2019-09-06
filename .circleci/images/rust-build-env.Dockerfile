# Rust build environment Dockerfile.
# NOTE: When making changes, also bumb the version number in
# the CircleCI config, or existing builds will break.
#
# See also the Rust playground Dockerfile: 
# https://github.com/integer32llc/rust-playground/blob/master/compiler/base/Dockerfile
#
# TODO: Automatically trigger a weekly rebuild with the latest stable.
FROM circleci/rust:1.37-node

# The latest nightly
# TODO: Update manually. 
ENV NIGHTLY="nightly-2019-08-15"

# Install clippy and sccache
RUN rustup component add clippy \
    && cargo install sccache --no-default-features \
    # Install Nightly with rustfmt, wasm and Cortex-M3 support
    && rustup toolchain install $NIGHTLY \
    && rustup target add wasm32-unknown-unknown --toolchain $NIGHTLY \
    && rustup target add thumbv7m-none-eabi --toolchain $NIGHTLY \
    && rustup component add rustfmt --toolchain $NIGHTLY \
    # Install wasm-gc and twiggy
    && cargo install --git https://github.com/alexcrichton/wasm-gc \
    && cargo install twiggy \
    # For the rocksdb dependency of substrate-node
    && sudo apt-get install clang

# Compile project to load up global cargo caches.
# TODO: Automatically trigger a weekly rebuild for the latest versions
# TODO: Also preserve target folder
COPY --chown=circleci:circleci . /project
RUN cd /project \
    && whoami \
    && cargo build --all --all-targets --all-features \
    && cargo build --release --all --all-targets --all-features \
    && mv /project/target /root \
    && rm -r /project

RUN cargo install cargo-cache \
 && cargo cache --autoclean-expensive
