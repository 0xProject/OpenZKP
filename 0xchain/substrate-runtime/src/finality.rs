// TODO - Add method system to add or remove validators [and look into substrate
// native modules like democracy]
use crate::wrappers::*;
use openstark::{
    check_proof,
    fibonacci::{get_fibonacci_constraints, PublicInput},
    ProofParams,
};
use parity_codec::{Decode, Encode};
use rstd::prelude::*;
#[cfg(feature = "std")]
use runtime_io::{with_storage, ChildrenStorageOverlay, StorageOverlay};
use support::{
    decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageMap, StorageValue,
};
use system::ensure_signed;

#[derive(PartialEq, Eq, Encode, Default, Clone, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct RecordedProof {
    proof:  Vec<u8>,
    public: Vec<u8>,
}

pub trait Trait: system::Trait {
    type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

// TODO - A linked list analogue is much better suited to this than a mapping
decl_storage! {
    trait Store for Module<T: Trait> as FinalityProof {
        // TODO - Change the hash to be a totally empty vault tree
        VaultChain get(get_vault_hash) build(|_config: &GenesisConfig<T>| {vec![(0_u32, [0; 32])]}) : map u32 => [u8; 32]; // Records each vault hash status
        VoteCount  get(get_vote_count) : (u32, u32) = (0, 0); // (number for, number against)
        MaxIndex get(get_max_index): u32 = 1;

        LinkTop get(get_current_link) : u32 = 1; // The index which has the end of the linked list
        LinkedHash get(get_hash_link) build(|_config: &GenesisConfig<T>| {vec![(0_u32, 0_u32)]}): map u32 => u32; // Contains a reference to the id of each link in the list
        NextProof get(get_ready_for_proof): bool = true;

        Validators get(get_validator_key): map T::AccountId => PublicKey;
        // TODO - Figure out why the extra genesis doesn't override the default
        NumValidators get(get_number_of_validators): u32 = 3;
    }
        // Used for testing
    add_extra_genesis {
        config(authorities): Vec<(T::AccountId, PublicKey)>;

        build(|storage: &mut StorageOverlay, _: &mut ChildrenStorageOverlay, config: &GenesisConfig<T>| {
            with_storage(storage, || {
                for (ref acct, ref key) in &config.authorities {
                    let _ = <Module<T>>::add_validator(acct.clone(), key.clone());
                }
            });
        });
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        // TODO - Want this to take a url pull or a ipfs hash to reduce node size
        pub fn prove_chain(origin, recorded: RecordedProof, end_index: u32) -> Result {
            let _ = ensure_signed(origin)?; //TODO - Do we need this? Do we want to lock to validators or add a cost?
            ensure!(<NextProof<T>>::get(), "Voting not open");
            ensure!(end_index < <MaxIndex<T>>::get(), "This end hash hasn't been recorded yet");

            let last_link = <LinkTop<T>>::get();
            let start_index = <LinkedHash<T>>::get(last_link-1);
            let _start_hash = <VaultChain<T>>::get(start_index);
            let _end_hash = <VaultChain<T>>::get(end_index);

            //TODO - This will be sliced out for a starkdex proof.
            let public : PublicInput = recorded.public.as_slice().into();
            ensure!(check_proof(
                recorded.proof.as_slice(),
                &get_fibonacci_constraints(&public),
                &public,
                & ProofParams {
                blowup:                   16,
                pow_bits:                 12,
                queries:                  20,
                fri_layout:               vec![3, 2],
                constraints_degree_bound: 1,
            },
                2,
                1024
            ), "The proof is invalid");

            <NextProof<T>>::put(false);
            <LinkedHash<T>>::insert(last_link, end_index);
            Self::deposit_event(Event::ProofRecorded(start_index, end_index, recorded));
            Ok(())
        }

        pub fn issue_signature(origin, sig: Signature) -> Result {
            ensure!(!<NextProof<T>>::get(), "Voting not open");
            let who = ensure_signed(origin)?;
            ensure!(<Validators<T>>::exists(who.clone()), "Called by non validator");

            let public = <Validators<T>>::get(who);
            let last_link = <LinkTop<T>>::get();
            let which = <LinkedHash<T>>::get(last_link);
            let vault_hash = <VaultChain<T>>::get(which);

            ensure!(verify(vault_hash, &sig, &public), "Invalid Signature");
            Self::deposit_event(Event::Signed(which, vault_hash, sig, public));
            let mut status : (u32, u32) = <VoteCount<T>>::get();
            status.0 += 1_u32;
            <VoteCount<T>>::put(status);
            Self::check_finality(which, &mut status, vault_hash);

            Ok(())
        }

        pub fn issue_rebuke(origin) -> Result {
            ensure!(!<NextProof<T>>::get(), "Voting not open");
            let who = ensure_signed(origin)?;
            ensure!(<Validators<T>>::exists(who.clone()), "Called by non validator");

            let public = <Validators<T>>::get(who);
            let last_link = <LinkTop<T>>::get();
            let which = <LinkedHash<T>>::get(last_link);
            let vault_hash = <VaultChain<T>>::get(which);

            Self::deposit_event(Event::Rebuked(which, vault_hash, public));
            let mut status : (u32, u32) = <VoteCount<T>>::get();
            status.1 += 1_u32;
            <VoteCount<T>>::put(status);
            Self::check_finality(which, &mut status, vault_hash);

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    fn check_finality(which: u32, status: &mut (u32, u32), vault_hash: [u8; 32]) {
        let number_of_validators = <NumValidators<T>>::take();
        if status.0 + status.1 == number_of_validators {
            // TODO - Abstract as adjustable constants [it is set as 2/3 approval needed
            // right now]
            let last_link = <LinkTop<T>>::get();
            if 3 * status.0 >= 2 * number_of_validators {
                Self::deposit_event(Event::Finalized(which, vault_hash));
                // Clears out unneeded hashes and resets to allow next proof.
                let previous = <LinkedHash<T>>::get(last_link - 1);
                for hash_index in previous..which {
                    // Removes the unneeded hashes from runtime memory.
                    <VaultChain<T>>::remove(hash_index);
                }
                // Moves the chain forward
                <LinkTop<T>>::put(last_link + 1);
            } else {
                Self::deposit_event(Event::Failed(which, vault_hash));
            }
            // Remove the vote count from runtime memory.
            // Note - Killing the vote count resets it back to (0,0)
            <VoteCount<T>>::kill();
            // Allows another proof submission
            <NextProof<T>>::put(true);
            // Clear the unneeded link pointer
            <LinkedHash<T>>::remove(last_link);
        }
    }

    pub fn add_hash(vault_hash: [u8; 32]) {
        let index = <MaxIndex<T>>::take();
        <VaultChain<T>>::insert(index, vault_hash);
        <MaxIndex<T>>::put(index + 1);
    }

    // TODO - When we figure out other ways to add validators we can remove this
    #[cfg(feature = "std")]
    fn add_validator(substrate_key: T::AccountId, stark_key: PublicKey) -> Result {
        let num = <NumValidators<T>>::get();
        <NumValidators<T>>::put(num + 1);
        <Validators<T>>::insert(substrate_key, stark_key);
        Ok(())
    }

    // TODO - Weird flag about this being unused when it shouldn't be
    #[cfg(feature = "std")]
    #[allow(dead_code)]
    fn open_proof_at(which: u32) {
        <NextProof<T>>::put(false);
        <LinkedHash<T>>::insert(<LinkTop<T>>::get(), which);
    }
}

// TODO - Replace with a hash type from the merkle system in stark.
type HashContainer = [u8; 32]; // We need a type alias to avoid brackets in the macro.

decl_event! {
    pub enum Event
    {
        ProofRecorded(u32, u32, RecordedProof),
        Signed(u32, HashContainer, Signature, PublicKey),
        Rebuked(u32, HashContainer, PublicKey),
        Finalized(u32, HashContainer),
        Failed(u32, HashContainer),
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
    use starkdex::wrappers::{public_key, sign};
    use support::{assert_ok, impl_outer_origin};
    use u256::U256;

    impl_outer_origin! {
        pub enum Origin for FinalityTest {}
    }

    // For testing the module, we construct most of a mock runtime. This means
    // first constructing a configuration type (`Test`) which `impl`s each of the
    // configuration traits of modules we want to use.
    #[derive(Clone, Eq, PartialEq)]
    pub struct FinalityTest;
    impl system::Trait for FinalityTest {
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
    impl Trait for FinalityTest {
        type Event = ();
    }
    type FinalityProof = Module<FinalityTest>;

    // This function basically just builds a genesis storage key/value store
    // according to our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        let paul_private =
            u256h!("02c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let remco_private =
            u256h!("04c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let mason_private =
            u256h!("05c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let paul_public: PublicKey = public_key(&paul_private.to_bytes_be()).into();
        let remco_public: PublicKey = public_key(&remco_private.to_bytes_be()).into();
        let mason_public: PublicKey = public_key(&mason_private.to_bytes_be()).into();

        let mut t = system::GenesisConfig::<FinalityTest>::default()
            .build_storage()
            .unwrap()
            .0;
        t.extend(
            GenesisConfig::<FinalityTest> {
                authorities: vec![(0, paul_public), (1, remco_public), (2, mason_public)],
            }
            .build_storage()
            .unwrap()
            .0,
        );
        t.into()
    }

    #[test]
    fn proof_posts_right() {
        let public = PublicInput {
            index: 1000,
            value: FieldElement::from(u256h!(
                "0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f"
            )),
        };
        let private = PrivateInput {
            secret: FieldElement::from(u256h!(
                "00000000000000000000000000000000000000000000000000000000cafebabe"
            )),
        };
        let constraints = &get_fibonacci_constraints(&public);
        let actual = stark_proof(
            &get_trace_table(1024, &private),
            &constraints,
            &public,
            &ProofParams {
                blowup:                   16,
                pow_bits:                 12,
                queries:                  20,
                fri_layout:               vec![3, 2],
                constraints_degree_bound: 1,
            },
        );

        with_externalities(&mut new_test_ext(), || {
            FinalityProof::add_hash([1; 32]);
            // Note duplicates are possible in block hash [substrate blocks confirm fast so
            // may not include any tx]
            FinalityProof::add_hash([1; 32]);
            FinalityProof::add_hash([2; 32]);
            FinalityProof::add_hash([3; 32]);
            assert_ok!(FinalityProof::prove_chain(
                Origin::signed(0),
                RecordedProof {
                    proof:  actual.proof.clone(),
                    public: (&public).into(),
                },
                3
            ));
        })
    }
    #[test]
    fn updates_properly_with_approval() {
        let paul_private =
            u256h!("02c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let remco_private =
            u256h!("04c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let mason_private =
            u256h!("05c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");

        let hash_1 = hash([1; 32], [1; 32]);
        let hash_2 = hash([2; 32], [2; 32]);
        let paul_sig = sign(&hash_2, &paul_private.to_bytes_be());
        let remco_sig = sign(&hash_2, &remco_private.to_bytes_be());
        let mason_sig = sign(&hash_2, &mason_private.to_bytes_be());

        with_externalities(&mut new_test_ext(), || {
            FinalityProof::add_hash(hash_1);
            // Note duplicates are possible in block hash [substrate blocks confirm fast so
            // may not include any tx]
            FinalityProof::add_hash(hash_1);
            FinalityProof::add_hash(hash_2);
            FinalityProof::open_proof_at(3);
            assert_ok!(FinalityProof::issue_signature(
                Origin::signed(0),
                paul_sig.into()
            ));
            assert_ok!(FinalityProof::issue_signature(
                Origin::signed(1),
                remco_sig.into()
            ));
            assert_ok!(FinalityProof::issue_signature(
                Origin::signed(2),
                mason_sig.into()
            ));
            assert_eq!(FinalityProof::get_current_link(), 2);
            assert_eq!(FinalityProof::get_ready_for_proof(), true);
            assert_eq!(FinalityProof::get_vault_hash(2), [0; 32]);
            assert_eq!(FinalityProof::get_vault_hash(3), hash_2);
        })
    }
    #[test]
    fn rebukes_proofs_properly() {
        let paul_private =
            u256h!("02c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");

        let hash_1 = hash([1; 32], [1; 32]);
        let hash_2 = hash([2; 32], [2; 32]);
        let paul_sig = sign(&hash_2, &paul_private.to_bytes_be());

        with_externalities(&mut new_test_ext(), || {
            FinalityProof::add_hash(hash_1);
            // Note duplicates are possible in block hash [substrate blocks confirm fast so
            // may not include any tx]
            FinalityProof::add_hash(hash_1);
            FinalityProof::add_hash(hash_2);
            FinalityProof::open_proof_at(3);
            assert_ok!(FinalityProof::issue_signature(
                Origin::signed(0),
                paul_sig.into()
            ));
            assert_ok!(FinalityProof::issue_rebuke(Origin::signed(1)));
            assert_ok!(FinalityProof::issue_rebuke(Origin::signed(2)));
            assert_eq!(FinalityProof::get_current_link(), 1);
            assert_eq!(FinalityProof::get_ready_for_proof(), true);
            assert_eq!(FinalityProof::get_hash_link(1), 0);
            assert_eq!(FinalityProof::get_vault_hash(1), hash_1);
            assert_eq!(FinalityProof::get_vault_hash(2), hash_1);
            assert_eq!(FinalityProof::get_vault_hash(3), hash_2);
        })
    }
}
