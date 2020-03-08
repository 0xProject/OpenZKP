#![warn(clippy::all)]
use criterion::criterion_main;

mod fft;
mod field;
mod permute;
mod transpose;

criterion_main!(field::group, fft::group, transpose::group, permute::group);
