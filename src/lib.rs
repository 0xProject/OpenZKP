pub mod field;
pub mod curve;
mod pedersen;
mod pedersen_points;
mod ecdsa;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
