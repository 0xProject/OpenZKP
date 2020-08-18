# Rust build environment Dockerfile.
# NOTE: When making changes, also bumb the version number in
# the CircleCI config, or existing builds will break.
#
# See also the Rust playground Dockerfile: 
# https://github.com/integer32llc/rust-playground/blob/master/compiler/base/Dockerfile
FROM circleci/rust:1-node

# The latest nightly
# TODO: Update manually. 
ENV NIGHTLY="nightly-2020-07-26"

RUN true \
    # For coverage reports
    && sudo apt-get install lcov \
    # Update rust stable and use
    && rustup update stable \
    && rustup default stable \
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
    && cargo install cargo-hack \
    # More analysis tools
    && cargo install cargo-outdated \
    && cargo install cargo-audit \
    && cargo install cargo-geiger \
    && sudo apt-get install python3-pip \
    && python3 -m pip install remarshal --user \
    # Compress cargo caches
    && cargo cache --autoclean-expensive

# Flags used to build coverage. To benefit from precompiling, we need to use
# identical flags in CI, which is why they are exported in an ENV.
# See https://users.rust-lang.org/t/howto-generating-a-branch-coverage-report/8524
# NOTE: We could add `-Coverflow-checks=off` but we want overflow checks in unit tests.
ENV COVFLAGS="-Dwarnings -Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Copt-level=1"

# Compile project to load up global cargo caches.
# We also leave the `.git` and `target` folder around as this
# will speedup CI builds. The `checkout` routine will make sure
# we have a fresh source checkout in CI.
COPY --chown=circleci:circleci . /home/circleci/project

RUN true \
    # Fetch project dependencies
    && cd $HOME/project \
    && cargo fetch \
    # Compress cargo caches
    && cargo cache --autoclean-expensive

# Warnings are not accepted in CI build
ENV RUSTFLAGS="-Dwarnings"

# Prebuild all to fill caches
RUN true \
    && cd $HOME/project \
    && CARGO_INCREMENTAL=0 RUSTFLAGS="$COVFLAGS" cargo +$NIGHTLY t --no-run \
    && cargo lint \
    && cargo +$NIGHTLY nostd_all \
    && cargo perf_all --no-run
