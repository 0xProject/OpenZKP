use parity_codec::{Decode, Encode};
use primefield::FieldElement;
use rstd::prelude::*;
#[cfg(feature = "std")]
use runtime_io::{with_storage, ChildrenStorageOverlay, StorageOverlay};
use starkdex::wrappers::*;
use support::{decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageMap};
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

impl From<PublicKey> for ([u8; 32], [u8; 32]) {
    fn from(key: PublicKey) -> ([u8; 32], [u8; 32]) {
        (key.x, key.y)
    }
}

impl From<([u8; 32], [u8; 32])> for PublicKey {
    fn from(key: ([u8; 32], [u8; 32])) -> PublicKey {
        PublicKey { x: key.0, y: key.1 }
    }
}

impl From<Signature> for ([u8; 32], [u8; 32]) {
    fn from(key: Signature) -> ([u8; 32], [u8; 32]) {
        (key.r, key.s)
    }
}

impl From<([u8; 32], [u8; 32])> for Signature {
    fn from(key: ([u8; 32], [u8; 32])) -> Signature {
        Signature { r: key.0, s: key.1 }
    }
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
        Asset1 get(balance): map PublicKey => u32;
    }

    // Used for testing
    add_extra_genesis {
        config(owners): Vec<(T::AccountId, PublicKey, u32, u32)>;

        build(|storage: &mut StorageOverlay, _: &mut ChildrenStorageOverlay, config: &GenesisConfig<T>| {
            with_storage(storage, || {
                for (ref acct, ref key, nonce, balance) in &config.owners {
                    let _ = <Module<T>>::balance_set_up(acct.clone(), key.clone(), *nonce, *balance);
                }
            });
        });
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
            let hash = hash(&U256::from((u64::from(nonce) << 32)+ u64::from(amount)).to_bytes_be(), &to.x);

            ensure!(verify(&hash, (&sig.r, &sig.s), (&stark_sender.x, &stark_sender.y)), "Invalid Signature");

            let their_balance = <Asset1<T>>::get(to.clone());

             <Asset1<T>>::insert(stark_sender.clone(), balance - amount);
             <Asset1<T>>::insert(to.clone(), their_balance + amount);
             <Nonces<T>>::insert(stark_sender.clone(), nonce+1);
             Ok(())
        }

        pub fn register(origin, who: PublicKey, sig: Signature) -> Result
        {
            let sender = ensure_signed(origin)?;
            let mut data : Vec<u8> = sender.clone().encode();
            // TODO - We could hash the id here instead of padding, should we?
            for _ in 0..(32-data.len()) {
                data.push(0_u8);
            }
            let mut sized = [0_u8; 32];
            sized.copy_from_slice(data.as_slice());
            let field_version = FieldElement::from(U256::from_bytes_be(&sized));

            ensure!(verify(&field_version.as_montgomery().to_bytes_be(), (&sig.r, &sig.s), (&who.x, &who.y)), "Invalid Signature");

            <Asset1<T>>::insert(who.clone(), 0);
            <Nonces<T>>::insert(who.clone(), 0);
            <PublicKeys<T>>::insert(sender, who);
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    // Note this function is only used for testing and should never be made pub
    #[cfg(feature = "std")]
    fn balance_set_up(
        substrate_who: T::AccountId,
        who: PublicKey,
        balance: u32,
        nonce: u32,
    ) -> Result {
        <Asset1<T>>::insert(who.clone(), balance);
        <Nonces<T>>::insert(who.clone(), nonce);
        <PublicKeys<T>>::insert(substrate_who, who);
        Ok(())
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

    use macros_decl::u256h;
    use primitives::{Blake2Hasher, H256};
    use runtime_io::with_externalities;
    use runtime_primitives::{
        testing::{Digest, DigestItem, Header},
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };
    use support::{assert_ok, impl_outer_origin};

    impl_outer_origin! {
        pub enum Origin for TemplateTest {}
    }

    // For testing the module, we construct most of a mock runtime. This means
    // first constructing a configuration type (`Test`) which `impl`s each of the
    // configuration traits of modules we want to use.
    #[derive(Clone, Eq, PartialEq)]
    pub struct TemplateTest;
    impl system::Trait for TemplateTest {
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
    impl super::Trait for TemplateTest {
        type Event = ();
    }
    type TemplateModule = super::Module<TemplateTest>;

    // This function basically just builds a genesis storage key/value store
    // according to our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        let paul_private =
            u256h!("02c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let remco_private =
            u256h!("04c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let paul_public = public_key(&paul_private.to_bytes_be()).into();
        let remco_public = public_key(&remco_private.to_bytes_be()).into();

        let mut t = system::GenesisConfig::<TemplateTest>::default()
            .build_storage()
            .unwrap()
            .0;
        t.extend(
            GenesisConfig::<TemplateTest> {
                // Your genesis kitties
                owners: vec![(0, paul_public, 500, 50), (1, remco_public, 50, 0)],
            }
            .build_storage()
            .unwrap()
            .0,
        );
        t.into()
    }

    #[test]
    fn allows_registration() {
        let mut data: Vec<u8> = 111.encode(); // Note - In the substrate test environment account ids are u64 instead of
                                              // public keys
        for _ in 0..(32 - data.len()) {
            data.push(0_u8);
        }
        let mut sized = [0_u8; 32];
        sized.copy_from_slice(data.as_slice());
        let field_version = FieldElement::from(U256::from_bytes_be(&sized));

        let private_key =
            u256h!("03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");

        let sig = sign(
            &field_version.as_montgomery().to_bytes_be(),
            &private_key.to_bytes_be(),
        )
        .into();
        let public = public_key(&private_key.to_bytes_be()).into();
        with_externalities(&mut new_test_ext(), || {
            assert_ok!(TemplateModule::register(Origin::signed(111), public, sig));
        });
    }

    #[test]
    fn blocks_bad_registration() {
        let mut data: Vec<u8> = 111.encode(); // Note - In the substrate test environment account ids are u64 instead of
                                              // public keys
        for _ in 0..(32 - data.len()) {
            data.push(0_u8);
        }
        let mut sized = [0_u8; 32];
        sized.copy_from_slice(data.as_slice());
        let field_version = FieldElement::from(U256::from_bytes_be(&sized));
        let wrong_version = field_version + FieldElement::ONE;

        let private_key =
            u256h!("03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");

        let sig = sign(
            &wrong_version.as_montgomery().to_bytes_be(),
            &private_key.to_bytes_be(),
        )
        .into();
        let public = public_key(&private_key.to_bytes_be()).into();
        with_externalities(&mut new_test_ext(), || {
            assert_eq!(
                TemplateModule::register(Origin::signed(111), public, sig),
                Err("Invalid Signature")
            );
        });
    }

    #[test]
    fn allows_owner_to_move() {
        let paul_private =
            u256h!("02c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let remco_private =
            u256h!("04c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let paul_public: PublicKey = public_key(&paul_private.to_bytes_be()).into();
        let remco_public: PublicKey = public_key(&remco_private.to_bytes_be()).into();

        let hash = hash(
            &U256::from(((50_u64) << 32) + 40).to_bytes_be(),
            &remco_public.x,
        );
        let sig = sign(&hash, &paul_private.to_bytes_be()).into();
        with_externalities(&mut new_test_ext(), || {
            assert_ok!(TemplateModule::send_tokens(
                Origin::signed(0),
                40,
                remco_public.clone(),
                sig
            ));
            assert_eq!(TemplateModule::balance(remco_public), 90);
            assert_eq!(TemplateModule::balance(paul_public), 460);
        });
    }
}
