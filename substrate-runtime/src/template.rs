use macros_decl::u256h;
use primefield::FieldElement;
use rstd::prelude::*;
use parity_codec::{Decode, Encode};
use stark::{
    check_proof,
    fibonacci::{get_fibonacci_constraints, get_trace_table, PrivateInput, PublicInput},
    stark_proof, ProofParams,
};
#[allow(unused_imports)] // TODO - Remove when used
use starkdex::wrappers::*;
use support::{ensure, decl_event, decl_module, decl_storage, dispatch::Result, StorageValue, StorageMap};
use system::ensure_signed;
use u256::U256;

#[derive(PartialEq, Encode, Default, Clone, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct PublicKey {
    x: [u8; 32],
    y: [u8; 32],
}

#[derive(PartialEq, Encode, Default, Clone, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Signature {
    r: [u8; 32], 
    s: [u8; 32],
}

/// The module's configuration trait.
pub trait Trait: system::Trait {
    // TODO: Add other types and constants required configure this module.

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        PublicKeys : map T::AccountId => PublicKey;
        Nonces : map PublicKey => u32;
        Asset1 : map PublicKey => u32;
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        // this is needed only if you are using events in your module
        fn deposit_event<T>() = default;

        pub fn send_tokens(origin, amount: u32, to: PublicKey, sig: Signature) -> Result {
            let sender = ensure_signed(origin)?;

            ensure!(<PublicKeys<T>>::exists(sender.clone()), "Sender not registered");
            ensure!(<Nonces<T>>::exists(to.clone()), "To address not registered");

            let stark_sender = <PublicKeys<T>>::get(sender);
            let nonce = <Nonces<T>>::get(stark_sender.clone());
            let balance = <Asset1<T>>::get(stark_sender.clone());
            ensure!(balance > amount, "You don't have enough token");
            
            // TODO - Hashes over generic slices or better packing.
            let hash_to = hash(&to.x, &to.y);
            let hash_amount = hash(&hash_to, &U256::from(amount).to_bytes_be());
            let hash_nonce = hash(&hash_amount,  &U256::from(nonce).to_bytes_be());

            ensure!(verify(&hash_nonce, (&stark_sender.x, &stark_sender.y), (&sig.r, &sig.s)), "Invalid Signature");

            let their_balance = <Asset1<T>>::get(to.clone());

             <Asset1<T>>::insert(stark_sender.clone(), balance - amount);
             <Asset1<T>>::insert(to.clone(), their_balance + amount);
             <Nonces<T>>::insert(stark_sender.clone(), nonce+1);
             Ok(())
        }

        pub fn register(origin, who: PublicKey, sig: Signature) -> Result 
        {
            let sender = ensure_signed(origin)?;
            let data : Vec<u8> = sender.clone().encode();
            let mut sized = [0_u8; 32];
            sized.copy_from_slice(data.as_slice());

            ensure!(verify(&sized, (&who.x, &who.y), (&sig.r, &sig.s)), "Invalid Signature");

            <Asset1<T>>::insert(who.clone(), 0);
            <Nonces<T>>::insert(who.clone(), 0);
            <PublicKeys<T>>::insert(sender, who);
            Ok(())
        }
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        // Just a dummy event.
        // Event `Something` is declared with a parameter of the type `u32` and `AccountId`
        // To emit this event, we call the deposit funtion, from our runtime funtions
        SomethingStored(usize, AccountId),
    }
);

/// tests for this module
#[cfg(test)]
mod tests {
    use super::*;

    use primitives::{Blake2Hasher, H256};
    use runtime_io::with_externalities;
    use runtime_primitives::{
        testing::{Digest, DigestItem, Header},
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };
    use support::{assert_ok, impl_outer_origin};

    impl_outer_origin! {
        pub enum Origin for Test {}
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
    impl Trait for Test {
        type Event = ();
    }
    type TemplateModule = Module<Test>;

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
    fn it_works_for_default_value() {
        with_externalities(&mut new_test_ext(), || {
            // Just a dummy test for the dummy funtion `do_something`
            // calling the `do_something` function with a value 42
            assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
            // asserting that the stored value is equal to what we stored
            // assert_eq!(TemplateModule::something(), Some(42));
        });
    }
}
