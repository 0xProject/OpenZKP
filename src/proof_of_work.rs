use crate::channel::*;
use crate::u256::*;
use rayon::prelude::*;
use tiny_keccak::Keccak;
use hex_literal::*;

pub fn pow_find_nonce(pow_bits: u32, proof: &Channel) -> u64 {
    let mut seed = hex!("0123456789abcded").to_vec();
    seed.extend_from_slice(&proof.digest);
    for byte in pow_bits.to_be_bytes().iter() {
        if *byte > 0 {
            seed.push(*byte);
            break;
        }
    }
    let mut seed_res = [0_u8; 32];
    let mut sha3 = Keccak::new_keccak256();
    sha3.update(&seed);
    sha3.finalize(&mut seed_res);

    let test_value = U256::from(2_u64).pow(u64::from(256 - pow_bits)).unwrap();
    for n in 0..(u64::max_value() as usize) {
        let mut sha3 = Keccak::new_keccak256();
        let mut res = [0; 32];
        sha3.update(&seed_res);
        sha3.update(&(n.to_be_bytes()));
        sha3.finalize(&mut res);
        let final_int = U256::from_bytes_be(&res);
        if final_int.leading_zeros() == pow_bits as usize && final_int < test_value {
            // Only do the large int compare if the quick logs match
            return n as u64;
        }
    }
    0
}

// TODO - Make tests compatible with the proof of work values from this function
pub fn pow_find_nonce_threaded(pow_bits: u32, proof: &Channel) -> u64 {
    let mut seed = hex!("0123456789abcded").to_vec();
    seed.extend_from_slice(&proof.digest);
    for byte in pow_bits.to_be_bytes().iter() {
        if *byte > 0 {
            seed.push(*byte);
            break;
        }
    }
    let mut seed_res = [0_u8; 32];
    let mut sha3 = Keccak::new_keccak256();
    sha3.update(&seed);
    sha3.finalize(&mut seed_res);

    let test_value = U256::from(2_u64).pow(u64::from(256 - pow_bits)).unwrap();
    let ret = (0..(u64::max_value() as usize))
        .into_par_iter()
        .find_any(|n| -> bool {
            let mut sha3 = Keccak::new_keccak256();
            let mut res = [0; 32];
            sha3.update(&seed_res);
            sha3.update(&(n.to_be_bytes()));
            sha3.finalize(&mut res);
            let final_int = U256::from_bytes_be(&res);
            if final_int.leading_zeros() == pow_bits as usize {
                final_int < test_value
            } else {
                false
            }
        });
    ret.unwrap() as u64
}

pub fn pow_verify(n: u64, pow_bits: u32, proof: &Channel) -> bool {
    let mut seed = hex!("0123456789abcded").to_vec();
    seed.extend_from_slice(&proof.digest);
    for byte in pow_bits.to_be_bytes().iter() {
        if *byte > 0 {
            seed.push(*byte);
            break;
        }
    }
    let mut seed_res = [0_u8; 32];
    let mut sha3 = Keccak::new_keccak256();
    sha3.update(&seed);
    sha3.finalize(&mut seed_res);

    let test_value = U256::from(2_u64).pow(u64::from(256 - pow_bits)).unwrap();
    let mut sha3 = Keccak::new_keccak256();
    let mut res = [0; 32];
    sha3.update(&seed_res);
    sha3.update(&(n.to_be_bytes()));
    sha3.finalize(&mut res);
    let final_int = U256::from_bytes_be(&res);
    final_int < test_value
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::channel::*;
    use crate::fibonacci::*;
    use crate::field::*;
    use crate::u256::U256;
    use crate::u256h;
    use hex_literal::*;

    #[test]
    fn proof_of_work_test() {
        let rand_source = Channel::new(
                hex!("0123456789abcded").to_vec().as_slice(),
        );
        let work = pow_find_nonce(15, &rand_source);
        assert!(pow_verify(work, 15, &rand_source));
    }

    #[test]
    fn threaded_proof_of_work_test() {
        let rand_source = Channel::new(
           hex!("0123456789abcded").to_vec().as_slice(),
        );
        let work = pow_find_nonce_threaded(15, &rand_source);
        assert!(pow_verify(work, 15, &rand_source));
    }
}
