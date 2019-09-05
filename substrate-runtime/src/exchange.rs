// Substrate needs a large enum but we can't put this directly on its
// declaration inside the substrate macro
#![allow(clippy::large_enum_variant)]

use crate::{finality, wrappers::*};
use macros_decl::hex;
use parity_codec::Encode;
use primefield::FieldElement;
use rstd::prelude::*;
#[cfg(feature = "std")]
use runtime_io::{with_storage, ChildrenStorageOverlay, StorageOverlay};
use runtime_primitives::traits::{BlakeTwo256, Hash};
use support::{decl_module, decl_storage, dispatch::Result, ensure, StorageMap, StorageValue};
use system::{ensure_root, ensure_signed};
use u256::U256;

/// The module's configuration trait.
pub trait Trait: finality::Trait {}

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as Exchange {
        pub PublicKeys : map T::AccountId => PublicKey;
        pub Nonces : map PublicKey => u32;
        pub Asset1 get(balance): map PublicKey => u32;
        pub Vaults get(get_vault): map u32 => Vault;
        pub ExecutedIds get(is_executed): map u32 => bool;
        // The available id is the index of the tree which has the most recently freed value
        AvailableID: u32 = 0;
        // Contains a listing of freed ids, and at key 0 a max id released which is known to always be free
        RecycledID build(|_config: &GenesisConfig<T>| {vec![(0_u32,0_u32)]}): map u32 => u32;
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
        // This is an example of our sig verification but will be eventually removed
        pub fn send_tokens(origin, amount: u32, to: PublicKey, sig: Signature) -> Result {
            let sender = ensure_signed(origin)?;

            ensure!(<PublicKeys<T>>::exists(sender.clone()), "Sender not registered");
            ensure!(<Nonces<T>>::exists(to.clone()), "To address not registered");

            let stark_sender = <PublicKeys<T>>::get(sender);
            let nonce = <Nonces<T>>::get(stark_sender.clone());
            let balance = <Asset1<T>>::get(stark_sender.clone());
            ensure!(balance > amount, "You don't have enough token");

            let hash = hash(U256::from((u64::from(nonce) << 32)+ u64::from(amount)).to_bytes_be(), to.x);

            ensure!(verify(hash, &sig, &stark_sender), "Invalid Signature");

            let their_balance = <Asset1<T>>::get(to.clone());

             <Asset1<T>>::insert(stark_sender.clone(), balance - amount);
             <Asset1<T>>::insert(to.clone(), their_balance + amount);
             <Nonces<T>>::insert(stark_sender.clone(), nonce+1);
             Ok(())
        }

        pub fn register(origin, who: PublicKey, sig: Signature) -> Result {
            let sender = ensure_signed(origin)?;
            let data : Vec<u8> = sender.clone().encode();
            let hash : [u8; 32] = BlakeTwo256::hash_of(&data).into();
            let field_version = FieldElement::from(U256::from_bytes_be(&hash));

            ensure!(verify(field_version.as_montgomery().to_bytes_be(), &sig, &who), "Invalid Signature");

            <Asset1<T>>::insert(who.clone(), 0);
            <Nonces<T>>::insert(who.clone(), 0);
            <PublicKeys<T>>::insert(sender, who);
            Ok(())
        }

        pub fn setup_vault(origin, token_id: [u8; 24]) -> Result {
            let sender = ensure_signed(origin)?;
            ensure!(<PublicKeys<T>>::exists(sender.clone()), "Sender not registered");
            let stark_sender = <PublicKeys<T>>::get(sender);

            let next = <AvailableID<T>>::take();

            if next == 0 {
                let id = <RecycledID<T>>::get(next);
                <Vaults<T>>::insert(id, Vault{
                    owner: stark_sender,
                    token_id: token_id,
                    balance: 0,
                });
                <RecycledID<T>>::insert(0, id+1);
                Ok(())
            } else {
                let id = <RecycledID<T>>::get(next);
                <Vaults<T>>::insert(id, Vault{
                    owner: stark_sender,
                    token_id: token_id,
                    balance: 0,
                });
                <AvailableID<T>>::put(next-1);
                Ok(())
            }
        }

        // An authorized deposit creation function through which a super user can make deposits.
        // TODO - We don't want this long term, we want some type of test which shows that the deposited info is on ethereum.
        // TODO - When adding block proofs we can require this shows up in the deposit proof.
        pub fn deposit_authorized(origin, vault_id: u32, amount: u64) -> Result {
            ensure_root(origin)?;
            ensure!(<Vaults<T>>::exists(vault_id), "User should register first");
            let mut data = <Vaults<T>>::get(vault_id);
            data.balance += amount;
            Ok(())
        }

        // TODO - Edge case grief-ing where you can just pick another trade id and someone else's order won't go through, really should be hashes.
        pub fn execute_order(origin, order: TakerMessage, sig: Signature) -> Result {
            let sender = ensure_signed(origin)?;
            ensure!(<PublicKeys<T>>::exists(sender.clone()), "Sender not registered");
            let stark_sender = <PublicKeys<T>>::get(sender);

            // Checks that this hasn't been executed
            ensure!(!<ExecutedIds<T>>::exists(order.maker_message.trade_id), "Trade has already been executed");

            ensure!(<Vaults<T>>::exists(order.vault_a), "Missing taker vault a");
            ensure!(<Vaults<T>>::exists(order.vault_b), "Missing taker vault b");
            ensure!(<Vaults<T>>::exists(order.maker_message.vault_a), "Missing maker vault a");
            ensure!(<Vaults<T>>::exists(order.maker_message.vault_b), "Missing maker vault b");

            let mut taker_vault_a = <Vaults<T>>::get(order.vault_a);
            let mut taker_vault_b = <Vaults<T>>::get(order.vault_b);
            ensure!(taker_vault_a.owner == taker_vault_b.owner, "Mismatch in taker vault owners");

            let mut maker_vault_a = <Vaults<T>>::get(order.maker_message.vault_a);
            let mut maker_vault_b = <Vaults<T>>::get(order.maker_message.vault_b);
            // Check the vaults are owned by the same key
            ensure!(maker_vault_a.owner == maker_vault_b.owner, "Mismatch in maker vault owners");
            // Check that the vault asset types are what is indicated
            ensure!(order.maker_message.token_a == maker_vault_a.token_id, "Token in maker order vault_a doesn't match vault token type");
            ensure!(order.maker_message.token_b == maker_vault_b.token_id, "Token in maker order vault_a doesn't match vault token type");

            // Checks that the a vaults and be vaults have the same asset types across maker and taker.
            ensure!(maker_vault_a.token_id == taker_vault_a.token_id, "Mismatched token types");
            ensure!(maker_vault_b.token_id == taker_vault_a.token_id, "Mismatched token types");

            // Checks that we can transfer amount_a of tokens from maker and amount_b of tokens from taker
            ensure!(order.maker_message.amount_a <= maker_vault_a.balance, "Not enough funds in maker's source vault");
            ensure!(order.maker_message.amount_b <= taker_vault_b.balance, "Not enough funds in taker's source vault");

            // Verifies the starkdex signature
            ensure!(maker_verify(order.maker_message.clone(), &order.maker_message.sig, &maker_vault_a.owner), "Maker message improperly signed");
            ensure!(taker_verify(order.clone(), &sig, &stark_sender), "Taker message improperly signed");

            // Moves amount_a from maker's vault_a to takers vault_a
            taker_vault_a.balance += order.maker_message.amount_a;
            maker_vault_a.balance -= order.maker_message.amount_a;
            ensure!(taker_vault_a.balance >= order.maker_message.amount_a, "Failed the overflow check");
            // Moves amount_b to maker's vault_b from taker's vault_b
            taker_vault_b.balance -= order.maker_message.amount_b;
            maker_vault_b.balance += order.maker_message.amount_b;
            ensure!(maker_vault_b.balance >= order.maker_message.amount_b, "Failed the overflow check");

            // If a vault is empty we delete it, and add it's id to the listing of available ones
            if maker_vault_a.balance == 0 {
                <Vaults<T>>::remove(order.maker_message.vault_a);
                let next = <AvailableID<T>>::take();
                <RecycledID<T>>::insert(next+1, order.maker_message.vault_a);
                <AvailableID<T>>::put(next+1);
            } else {
                <Vaults<T>>::insert(order.maker_message.vault_a, maker_vault_a);
            }
            if taker_vault_b.balance == 0 {
                <Vaults<T>>::remove(order.vault_b);
                let next = <AvailableID<T>>::take();
                <RecycledID<T>>::insert(next+1, order.vault_b);
                <AvailableID<T>>::put(next+1);
            } else {
                <Vaults<T>>::insert(order.vault_b, taker_vault_b);
            }

            // Updates these knowing that they will not be 0
            <Vaults<T>>::insert(order.vault_a, taker_vault_a);
            <Vaults<T>>::insert(order.maker_message.vault_b, maker_vault_b);
            <ExecutedIds<T>>::insert(order.maker_message.trade_id, true);
            Ok(())
        }

        pub fn withdraw(origin, vault_id: u32, amount: u64, sig: Signature) -> Result {
            let sender = ensure_signed(origin)?;
            ensure!(<PublicKeys<T>>::exists(sender.clone()), "Sender not registered");
            let stark_sender = <PublicKeys<T>>::get(sender);

            let mut sender_vault = <Vaults<T>>::get(vault_id);
            ensure!(sender_vault.owner == stark_sender, "Sender doesn't own the vault");
            ensure!(sender_vault.balance > amount, "Trying to withdraw too much");

            let nonce = <Nonces<T>>::get(stark_sender.clone());
            let hash = hash(((U256::from(u64::from(nonce)) << 64) + U256::from(amount)).to_bytes_be(), sender_vault.clone().vault_hash());
            ensure!(verify(hash, &sig, &stark_sender), "Invalid Signature");

            sender_vault.balance -= amount;
            <Nonces<T>>::insert(stark_sender, nonce+1);
            <Vaults<T>>::insert(vault_id, sender_vault);
            Ok(())
        }

        fn on_finalize() {
            let vaults_hash = Self::hash_balance_tree();
            <finality::Module<T>>::add_hash(vaults_hash);
        }
    }
}

// Hash of the zero vault ie hash(0, 0)
pub const EMPTY_VAULT_HASH: [u8; 32] =
    hex!("049ee3eba8c1600700ee1b87eb599f16716b0b1022947733551fde4050ca6804");

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

    fn empty_level(depth: u32) -> [u8; 32] {
        let mut level = EMPTY_VAULT_HASH;
        for _ in 0..depth {
            level = hash(level, level);
        }
        level
    }

    pub fn hash_balance_tree() -> [u8; 32] {
        let max_vault = <RecycledID<T>>::get(0);
        let power_of_two = max_vault.next_power_of_two();

        // Note since we are hashing the bottom row we are going to
        // TODO - Bad complexity around transitions from power domains of two
        let mut layer: Vec<[u8; 32]> = (0..power_of_two)
            .map(|x| {
                if x > max_vault {
                    EMPTY_VAULT_HASH
                } else {
                    <Vaults<T>>::get(x).vault_hash()
                }
            })
            .collect();

        let depth = 32 - power_of_two.leading_zeros();
        for _ in 0..depth {
            layer = layer
                .chunks(2)
                .map(|chunk| hash(chunk[0], chunk[1]))
                .collect();
        }
        debug_assert_eq!(layer.len(), 1);
        let mut value_subtree = layer[0];
        let mut other_half = Self::empty_level(depth);

        if depth < 31 {
            // Our tree is only hashed up to depth 31
            for _ in 0..(31 - depth) {
                value_subtree = hash(value_subtree, other_half);
                other_half = hash(other_half, other_half);
            }
        }

        value_subtree
    }
}

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
    use starkdex::wrappers::{public_key, sign};
    use support::{assert_ok, impl_outer_origin};

    impl_outer_origin! {
        pub enum Origin for ExchangeTest {}
    }

    // For testing the module, we construct most of a mock runtime. This means
    // first constructing a configuration type (`Test`) which `impl`s each of the
    // configuration traits of modules we want to use.
    #[derive(Clone, Eq, PartialEq)]
    pub struct ExchangeTest;
    impl system::Trait for ExchangeTest {
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
    impl finality::Trait for ExchangeTest {
        type Event = ();
    }
    impl Trait for ExchangeTest {}
    type Exchange = Module<ExchangeTest>;

    // This function basically just builds a genesis storage key/value store
    // according to our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        let paul_private =
            u256h!("02c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let remco_private =
            u256h!("04c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let paul_public = public_key(&paul_private.to_bytes_be()).into();
        let remco_public = public_key(&remco_private.to_bytes_be()).into();

        let mut t = system::GenesisConfig::<ExchangeTest>::default()
            .build_storage()
            .unwrap()
            .0;
        t.extend(
            GenesisConfig::<ExchangeTest> {
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
        let data: Vec<u8> = (10_u64).encode(); // Note - In the substrate test environment account ids are u64 instead of
                                               // public keys
        let hashed: [u8; 32] = BlakeTwo256::hash_of(&data).into();
        let field_version = FieldElement::from(U256::from_bytes_be(&hashed));

        let private_key =
            u256h!("03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");

        let sig = sign(
            &field_version.as_montgomery().to_bytes_be(),
            &private_key.to_bytes_be(),
        )
        .into();
        let public = public_key(&private_key.to_bytes_be()).into();
        with_externalities(&mut new_test_ext(), || {
            assert_ok!(Exchange::register(Origin::signed(10), public, sig));
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
                Exchange::register(Origin::signed(111), public, sig),
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
            U256::from(((50_u64) << 32) + 40).to_bytes_be(),
            remco_public.x,
        );
        let sig = sign(&hash, &paul_private.to_bytes_be()).into();
        with_externalities(&mut new_test_ext(), || {
            assert_ok!(Exchange::send_tokens(
                Origin::signed(0),
                40,
                remco_public.clone(),
                sig
            ));
            assert_eq!(Exchange::balance(remco_public), 90);
            assert_eq!(Exchange::balance(paul_public), 460);
        });
    }
}
