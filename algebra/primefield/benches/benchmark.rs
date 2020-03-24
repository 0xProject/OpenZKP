#![warn(clippy::all)]
use criterion::Criterion;

mod fft;
mod field;
mod permute;
mod transpose;

fn main() {
    let crit = &mut Criterion::default().configure_from_args();
    field::group(crit);
    fft::group(crit);
    transpose::group(crit);
    permute::group(crit);
    crit.final_summary();
}
