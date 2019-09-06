FROM circleci/rust:1.37-node

# Install clippy and sccache
RUN rustup component add clippy
RUN cargo install sccache --no-default-features

# Install Nightly with rustfmt, wasm and Cortex-M3 support
ENV NIGHTLY="nightly-2019-08-15"
RUN rustup toolchain install $NIGHTLY
RUN rustup target add wasm32-unknown-unknown --toolchain $NIGHTLY
RUN rustup target add thumbv7m-none-eabi --toolchain $NIGHTLY
RUN rustup component add rustfmt --toolchain $NIGHTLY

# Install wasm-gc and twiggy
RUN cargo install --git https://github.com/alexcrichton/wasm-gc
RUN cargo install twiggy

# For the rocksdb dependency of substrate-node
RUN sudo apt-get install clang
