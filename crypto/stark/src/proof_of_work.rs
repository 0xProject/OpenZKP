#[cfg(all(feature = "std", feature = "prover"))]
use log::info;
#[cfg(all(feature = "std", feature = "prover"))]
use rayon::prelude::*;
use std::convert::TryFrom;
#[cfg(all(feature = "std", feature = "prover"))]
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use tiny_keccak::Keccak;
use zkp_macros_decl::hex;
use zkp_u256::{Binary, U256};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Debug))]
pub(crate) struct ChallengeSeed([u8; 32]);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Debug))]
pub(crate) struct Challenge {
    seed:       [u8; 32],
    difficulty: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Debug))]
pub(crate) struct Response {
    nonce: u64,
}

impl ChallengeSeed {
    pub(crate) fn from_bytes(seed: [u8; 32]) -> Self {
        Self(seed)
    }

    pub(crate) fn with_difficulty(self, difficulty: usize) -> Challenge {
        let mut seed = [0_u8; 32];
        let mut keccak = Keccak::new_keccak256();
        keccak.update(&hex!("0123456789abcded"));
        keccak.update(&self.0);
        keccak.update(&[u8::try_from(difficulty).unwrap()]);
        keccak.finalize(&mut seed);
        Challenge { difficulty, seed }
    }
}

impl Challenge {
    pub(crate) fn verify(&self, response: Response) -> bool {
        // TODO: return Result<()>
        // OPT: Inline Keccak256 and work directly on buffer using 'keccakf'
        let mut keccak = Keccak::new_keccak256();
        let mut digest = [0_u8; 32];
        keccak.update(&self.seed);
        keccak.update(&(response.nonce.to_be_bytes()));
        keccak.finalize(&mut digest);
        // OPT: Check performance impact of conversion
        let work = U256::from_bytes_be(&digest).leading_zeros();
        work >= self.difficulty
    }
}

#[cfg(feature = "prover")]
impl Challenge {
    #[cfg(not(feature = "std"))]
    pub(crate) fn solve(&self) -> Response {
        // We assume a nonce exists and will be found in reasonable time.
        info!(
            "Solving {} bit proof of work single-threaded.",
            self.difficulty
        );
        #[allow(clippy::maybe_infinite_iter)]
        (0_u64..)
            .map(|nonce| Response { nonce })
            .find(|&response| self.verify(response))
            .expect("No valid nonce found")
    }

    #[cfg(feature = "std")]
    pub(crate) fn solve(&self) -> Response {
        let num_threads = rayon::current_num_threads();
        info!(
            "Solving {} bit proof of work with {} threads.",
            self.difficulty, num_threads
        );
        let first_nonce = AtomicU64::new(u64::max_value());
        (0..num_threads as u64).into_par_iter().for_each(|offset| {
            for nonce in (offset..).step_by(num_threads) {
                if self.verify(Response { nonce }) {
                    let _ = fetch_min(&first_nonce, nonce);
                }
                if nonce >= first_nonce.load(Relaxed) {
                    break;
                }
            }
        });
        Response {
            nonce: first_nonce.into_inner(),
        }
    }
}

impl Response {
    pub(crate) fn from_nonce(nonce: u64) -> Self {
        Self { nonce }
    }

    pub(crate) fn nonce(self) -> u64 {
        self.nonce
    }
}

// TODO: Use `fetch_min` instead
// See https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.fetch_max
// This is pending https://github.com/rust-lang/rust/issues/48655
#[cfg(all(feature = "std", feature = "prover"))]
fn fetch_min(atom: &AtomicU64, value: u64) -> u64 {
    let mut prev = atom.load(Relaxed);
    while prev > value {
        match atom.compare_exchange_weak(prev, value, Relaxed, Relaxed) {
            Ok(_) => return value,
            Err(next_prev) => prev = next_prev,
        }
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proof_of_work_test() {
        let challenge = ChallengeSeed::from_bytes(hex!(
            "0123456789abcded0123456789abcded0123456789abcded0123456789abcded"
        ))
        .with_difficulty(8);
        let response = challenge.solve();
        assert_eq!(response.nonce, 138);
        assert!(challenge.verify(response));
    }
}
