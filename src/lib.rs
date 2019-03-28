pub mod curve;
pub mod ecdsa;
pub mod field;
pub mod pedersen;
mod pedersen_points;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
