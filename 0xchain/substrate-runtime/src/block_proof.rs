// Substrate macros use `Default::default()`. To allow this we need to
// allow the lint on the whole file scope.
#![allow(clippy::default_trait_access)]

// TODO - This code will be be reworked to incorporate it into a chain which
// doesn't lock blocks to proofs.
#[cfg(feature = "std")]
use inherents::ProvideInherentData;
use inherents::{InherentData, InherentIdentifier, IsFatalError, ProvideInherent, RuntimeString};
use parity_codec::{Decode, Encode};
use rstd::{prelude::*, result::Result};
use support::{decl_module, decl_storage, StorageValue};
use system::ensure_inherent;

use openstark::{
    check_proof,
    fibonacci::{get_fibonacci_constraints, PublicInput},
    ProofParams,
};

pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"tx0proof";

#[derive(PartialEq, Encode, Default, Clone, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct RecordedProof {
    proof:  Vec<u8>,
    public: Vec<u8>,
}

pub type InherentType = RecordedProof;

/// Errors that can occur while checking the timestamp inherent.
#[derive(Encode)]
#[cfg_attr(feature = "std", derive(Debug, Decode))]
pub enum InherentError {
    InvalidProof(RuntimeString),
    /// Some other error.
    Other(RuntimeString),
}

impl IsFatalError for InherentError {
    fn is_fatal_error(&self) -> bool {
        true
    }
}

impl InherentError {
    /// Try to create an instance ouf of the given identifier and data.
    #[cfg(feature = "std")]
    pub fn try_from(id: InherentIdentifier, data: &[u8]) -> Option<Self> {
        if id == INHERENT_IDENTIFIER {
            <Self as parity_codec::Decode>::decode(&mut &data[..])
        } else {
            None
        }
    }
}

pub trait ProofInherentData {
    fn proof_inherent_data(&self) -> Result<InherentType, RuntimeString>;
}

impl ProofInherentData for InherentData {
    fn proof_inherent_data(&self) -> Result<InherentType, RuntimeString> {
        self.get_data(&INHERENT_IDENTIFIER)
            .and_then(|r| r.ok_or_else(|| "Inherent data not found".into()))
    }
}

#[cfg(feature = "std")]
pub struct InherentDataProvider;

#[cfg(feature = "std")]
impl ProvideInherentData for InherentDataProvider {
    fn inherent_identifier(&self) -> &'static InherentIdentifier {
        &INHERENT_IDENTIFIER
    }

    fn provide_inherent_data(&self, inherent_data: &mut InherentData) -> Result<(), RuntimeString> {
        inherent_data.put_data(INHERENT_IDENTIFIER, &0)
    }

    fn error_to_string(&self, error: &[u8]) -> Option<String> {
        RuntimeString::decode(&mut &error[..]).map(Into::into)
    }
}

/// The module configuration trait
pub trait Trait: system::Trait {}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn set(origin, recorded: RecordedProof) {
            ensure_inherent(origin)?;
            assert!(!<Self as Store>::SetProof::exists(), "The proof must be updated only once in the block");

            let public : PublicInput = recorded.public.as_slice().into();

            let mut no_macro_vec = Vec::new();
            no_macro_vec.push(3);
            no_macro_vec.push(2);

            assert!(check_proof(
                recorded.proof.as_slice(),
                &get_fibonacci_constraints(&public),
                &public,
                // TODO - These params should be stored or provided instead of hardcoded
                &ProofParams {
                    blowup: 				  16,
                    pow_bits: 				  12,
                    queries:   				  20,
                    fri_layout:               no_macro_vec,
                    constraints_degree_bound: 1,
                },
                2,
                1024
            ), "The block proof is invalid");

            <Self as Store>::Proof::put(recorded);
            <Self as Store>::SetProof::put(true);
        }

        fn on_finalize() {
            assert!(<Self as Store>::SetProof::take(), "The proof must be set once in the block");
        }
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as BlockProof {
        pub Proof get(proof): RecordedProof;
        pub SetProof get(set_proof): bool;
    }
}

fn extract_inherent_data(data: &InherentData) -> Result<InherentType, RuntimeString> {
    data.get_data::<InherentType>(&INHERENT_IDENTIFIER)
        .map_err(|_| RuntimeString::from("Invalid inherent data encoding."))?
        .ok_or_else(|| "Inherent data is not provided.".into())
}

impl<T: Trait> ProvideInherent for Module<T> {
    type Call = Call<T>;
    type Error = InherentError;

    const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;

    fn create_inherent(data: &InherentData) -> Option<Self::Call> {
        let data = extract_inherent_data(data).expect("Error in extracting inherent data.");
        Some(Call::set(data))
    }

    fn check_inherent(call: &Self::Call, data: &InherentData) -> Result<(), Self::Error> {
        let t: RecordedProof = match call {
            Call::set(ref t) => t.clone(),
            _ => return Err(InherentError::Other("Call Failure".into())),
        };
        let data1 = extract_inherent_data(data).expect("Error in extracting inherent data.");

        if t != data1 {
            return Err(InherentError::Other(
                "Posted proof doesn't match proof provided to call".into(),
            ));
        }

        let public: PublicInput = t.public.as_slice().into();

        let mut no_macro_vec = Vec::new();
        no_macro_vec.push(3);
        no_macro_vec.push(2);

        if check_proof(
            t.proof.as_slice(),
            &get_fibonacci_constraints(&public),
            &public,
            &ProofParams {
                blowup:                   16,
                pow_bits:                 12,
                queries:                  20,
                fri_layout:               no_macro_vec,
                constraints_degree_bound: 1,
            },
            2,
            1024,
        ) {
            Ok(())
        } else {
            Err(InherentError::InvalidProof(
                "Posted proof doesn't pass verification".into(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use macros_decl::u256h;
    use openstark::{
        fibonacci::{get_fibonacci_constraints, get_trace_table, PrivateInput, PublicInput},
        stark_proof, ProofParams,
    };
    use primefield::FieldElement;
    use primitives::{Blake2Hasher, H256};
    use runtime_io::with_externalities;
    use runtime_primitives::{
        testing::{Digest, DigestItem, Header},
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };
    use support::impl_outer_origin;
    use system::RawOrigin;
    use u256::U256;

    // TODO: What is this for?
    impl_outer_origin! {
        pub enum Origin for Test { }
    }

    // For testing the module, we construct most of a mock runtime. This means
    // first constructing a configuration type (`Test`) which `impl`s each of the
    // configuration traits of modules we want to use.
    #[derive(Clone, Eq, PartialEq)]
    pub struct Test;
    impl system::Trait for Test {
        type AccountId = u64;
        type BlockNumber = u64;
        type Digest = Digest;
        type Event = ();
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type Header = Header;
        type Index = u64;
        type Log = DigestItem;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Origin = Origin;
    }
    impl Trait for Test {}
    type BlockProof = Module<Test>;

    // This function basically just builds a genesis storage key/value store
    // according to our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap()
            .0
            .into()
    }

    #[test]
    #[should_panic(expected = "The block proof is invalid")]
    fn inherent_with_invalid_proof() {
        let private = PrivateInput {
            secret: FieldElement::from(u256h!(
                "00000000000000000000000000000000000000000000000f00dbabe0cafebabe"
            )),
        };
        let tt = get_trace_table(1024, &private);
        let public = PublicInput {
            index: 1000,
            value: tt[(1000, 0)].clone(),
        };
        let actual = stark_proof(
            &get_trace_table(1024, &private),
            &get_fibonacci_constraints(&public),
            &public,
            &ProofParams {
                blowup:                   16,
                pow_bits:                 12,
                queries:                  20,
                fri_layout:               vec![3, 2],
                constraints_degree_bound: 1,
            },
        );
        let incorrect_public = PublicInput {
            index: 1001,
            value: tt[(1004, 0)].clone(),
        };

        with_externalities(&mut new_test_ext(), || {
            let _ = BlockProof::dispatch(
                Call::set(RecordedProof {
                    proof:  actual.proof.clone(),
                    public: (&incorrect_public).into(),
                }),
                RawOrigin::Inherent.into(),
            );
        });
    }
}
