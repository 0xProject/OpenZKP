//! Substrate Node Template CLI library.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

mod chain_spec;
mod cli;
mod service;

pub use substrate_cli::{error, IntoExit, VersionInfo};

fn run() -> cli::error::Result<()> {
    let version = VersionInfo {
        name:            "Substrate Node",
        commit:          env!("VERGEN_SHA_SHORT"),
        version:         env!("CARGO_PKG_VERSION"),
        executable_name: "substrate-node",
        author:          "Paul",
        description:     "substrate-node",
        support_url:     "support.anonymous.an",
    };
    cli::run(::std::env::args(), cli::Exit, version)
}

error_chain::quick_main!(run);
