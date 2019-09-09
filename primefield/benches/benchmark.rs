#![warn(clippy::all)]
use criterion::criterion_main;

mod fft;
mod field;

criterion_main!(field::field, fft::fft);
