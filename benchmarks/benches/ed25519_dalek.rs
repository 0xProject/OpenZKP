use criterion::{black_box, Bencher};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature};
use rand::rngs::OsRng;
use sha2::Sha512;

pub fn ed25519_dalek_verify(bench: &mut Bencher, _i: &()) {
    let mut csprng = OsRng::new().unwrap();
    let keypair = Keypair::generate::<Sha512, _>(&mut csprng);
    let msg: &[u8] = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let sig: Signature = keypair.sign::<Sha512>(msg);

    bench.iter(|| black_box(keypair.verify::<Sha512>(black_box(msg), black_box(&sig))))
}
