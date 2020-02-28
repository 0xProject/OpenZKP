#![warn(clippy::all)]
use criterion::criterion_main;

mod fft;
mod field;
mod transpose;

criterion_main!(field::field, fft::fft, transpose::group);
