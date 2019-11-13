mod constraints;
mod inputs;
mod oods_values;
mod pedersen;
mod pedersen_points;
mod periodic_columns;
// mod trace_table;

use env_logger;
// use log::info;
// use std::time::Instant;
// use zkp_macros_decl::{field_element, hex};
// use zkp_primefield::FieldElement;
// use zkp_stark::{prove, Provable};
// use zkp_u256::U256;

// Need to import to active the logging allocator
#[allow(unused_imports)]
use zkp_logging_allocator;

fn main() {
    env_logger::init();
}
