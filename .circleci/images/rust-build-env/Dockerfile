FROM circleci/rust:1.37-node

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
