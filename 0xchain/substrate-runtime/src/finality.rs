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
        VaultChain get(get_vault_hash) : map u32 => [u8; 32]; // Records each vault hash status
        VoteCount  get(get_vote_count) : map u32  => (u32, u32); // (number for, number against)
        Sigs get(get_signature) : map (u32, T::AccountId) => Signature;
        Validators get(get_validator_key): map T::AccountId => PublicKey;
        MaxIndex get(get_max_index): u32 = 0;
        NumValidators get(get_number_of_validators): u32 = 3;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        // TODO - Want this to take a url pull or a ipfs hash to reduce node size
        pub fn prove_chain(origin, recorded: RecordedProof, start_index: u32, end_index: u32) -> Result {
            let _ = ensure_signed(origin)?; //TODO - Do we need this? Do we want to lock to validators or add a cost?

            let _start_hash = <VaultChain<T>>::get(start_index);
            let _end_hash = <VaultChain<T>>::get(end_index);

            //TODO - This will be sliced out for a starkdex proof.
            let public : PublicInput = recorded.public.as_slice().into();
            assert!(check_proof(
                recorded.proof.as_slice(),
                &get_fibonacci_constraints(&public),
                &public,
                // TODO - These params should be stored or provided instead of hardcoded
                &ProofParams {
                    blowup: 				  16,
                    pow_bits: 				  12,
                    queries:   				  20,
                    fri_layout:               vec![2, 3],
                    constraints_degree_bound: 1,
                },
                2,
                1024
            ), "The proof is invalid");

            <VoteCount<T>>::insert(end_index, (0, 0));
            Self::deposit_event(Event::ProofRecorded(start_index, end_index, recorded));
            Ok(())
        }

        pub fn issue_signature(origin, which: u32, sig: Signature) -> Result {
            let who = ensure_signed(origin)?;
            ensure!(<Validators<T>>::exists(who.clone()), "Called by non validator");
            let max = <MaxIndex<T>>::take();
            ensure!(which < max, "Invalid block number");
            let public = <Validators<T>>::get(who);

            let vault_hash = <VaultChain<T>>::get(which);
            ensure!(verify(vault_hash, &sig, &public), "Invalid Signature");
            Self::deposit_event(Event::Signed(which, vault_hash, sig, public));

            ensure!(<VoteCount<T>>::exists(which), "Can't vote on proof for this endpoint");
            let mut status : (u32, u32) = <VoteCount<T>>::get(which);
            status.0 += 1_u32;
            <VoteCount<T>>::insert(which, status);
            Self::check_finality(which, &mut status, vault_hash);

            Ok(())
        }

        pub fn issue_rebuke(origin, which: u32) -> Result {
            let who = ensure_signed(origin)?;
            ensure!(<Validators<T>>::exists(who.clone()), "Called by non validator");
            let max = <MaxIndex<T>>::take();
            ensure!(which < max, "Invalid block number");
            let public = <Validators<T>>::get(who);

            let vault_hash = <VaultChain<T>>::get(which);
            Self::deposit_event(Event::Rebuked(which, vault_hash, public));

            ensure!(<VoteCount<T>>::exists(which), "Can't vote on proof for this endpoint");
            let mut status : (u32, u32) = <VoteCount<T>>::get(which);
            status.1 += 1_u32;
            <VoteCount<T>>::insert(which, status);
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
            if 3 * status.0 > 2 * number_of_validators {
                Self::deposit_event(Event::Finalized(which, vault_hash));
            } else {
                Self::deposit_event(Event::Failed(which, vault_hash));
            }
        }
    }

    pub fn add_hash(vault_hash: [u8; 32]) {
        let index = <MaxIndex<T>>::take();
        <VaultChain<T>>::insert(index, vault_hash);
        <MaxIndex<T>>::put(index + 1);
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
