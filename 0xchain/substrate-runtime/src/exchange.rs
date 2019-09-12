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

pub const SIZE_LIMIT: u32 = 1024; // The max number of transactions we reasonably expect in one proof

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as Exchange {
        PublicKeys : map T::AccountId => PublicKey;
        Nonces : map PublicKey => u32;
        Vaults get(get_vault): map u32 => Vault;
        ExecutedIds get(is_executed): map u32 => bool;
        // The available id is the index of the tree which has the most recently freed value
        AvailableID get(get_available_id): u32 = 0;
        // Contains a listing of freed ids, and at key 0 a max id released which is known to always be free
        RecycledID get(get_recycled_id) build(|_config: &GenesisConfig<T>| {vec![(0_u32,0_u32)]}): map u32 => u32;
        // Stores and holds the current number of transactions
        BlockTransactions get(current_size) : u32 = 0;
    }

    // Used for testing
    add_extra_genesis {
        config(owners): Vec<(T::AccountId, u32, u32, Vault)>;

        build(|storage: &mut StorageOverlay, _: &mut ChildrenStorageOverlay, config: &GenesisConfig<T>| {
            with_storage(storage, || {
                for (ref acct, vault_id, nonce, ref vault) in &config.owners {
                    let _ = <Module<T>>::vault_set_up(acct.clone(), *vault_id, *nonce, vault.clone());
                }
            });
        });
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Associates a default substrate key with a stark key.
        pub fn register(origin, who: PublicKey, sig: Signature) -> Result {
            let sender = ensure_signed(origin)?;
            let data : Vec<u8> = sender.clone().encode();
            let hash : [u8; 32] = BlakeTwo256::hash_of(&data).into();
            let field_version = FieldElement::from(U256::from_bytes_be(&hash));

            ensure!(verify(field_version.as_montgomery().to_bytes_be(), &sig, &who), "Invalid Signature");

            <Nonces<T>>::insert(who.clone(), 0);
            <PublicKeys<T>>::insert(sender, who);
            Ok(())
        }

        //TODO - Dos protection
        pub fn vault_registration(origin, token_id: [u8; 24]) -> Result {
            let sender = ensure_signed(origin)?;
            ensure!(<PublicKeys<T>>::exists(sender.clone()), "Sender not registered");
            let stark_sender = <PublicKeys<T>>::get(sender);

            let next = <AvailableID<T>>::get();

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
            let tx_count = <BlockTransactions<T>>::get();
            ensure!(tx_count< SIZE_LIMIT, "Full block");
            ensure!(<Vaults<T>>::exists(vault_id), "User should register first");
            let mut data = <Vaults<T>>::get(vault_id);
            data.balance += amount;
            <BlockTransactions<T>>::put(tx_count+1);
            Ok(())
        }

        // TODO - Edge case grief-ing where you can just pick another trade id and someone else's order won't go through, really should be hashes.
        pub fn execute_order(origin, order: TakerMessage, sig: Signature) -> Result {
            let sender = ensure_signed(origin)?;
            let tx_count = <BlockTransactions<T>>::get();
            ensure!(tx_count< SIZE_LIMIT, "Full block");

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
            ensure!(order.maker_message.token_b == maker_vault_b.token_id, "Token in maker order vault_b doesn't match vault token type");

            // Checks that the a vaults and be vaults have the same asset types across maker and taker.
            ensure!(maker_vault_a.token_id == taker_vault_a.token_id, "Mismatched token_a types");
            ensure!(maker_vault_b.token_id == taker_vault_b.token_id, "Mismatched token_b types");

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
                let next = <AvailableID<T>>::get();
                <RecycledID<T>>::insert(next+1, order.maker_message.vault_a);
                <AvailableID<T>>::put(next+1);
            } else {
                <Vaults<T>>::insert(order.maker_message.vault_a, maker_vault_a);
            }
            if taker_vault_b.balance == 0 {
                <Vaults<T>>::remove(order.vault_b);
                let next = <AvailableID<T>>::get();
                <RecycledID<T>>::insert(next+1, order.vault_b);
                <AvailableID<T>>::put(next+1);
            } else {
                <Vaults<T>>::insert(order.vault_b, taker_vault_b);
            }

            // Updates these knowing that they will not be 0
            <Vaults<T>>::insert(order.vault_a, taker_vault_a);
            <Vaults<T>>::insert(order.maker_message.vault_b, maker_vault_b);
            <ExecutedIds<T>>::insert(order.maker_message.trade_id, true);
            <BlockTransactions<T>>::put(tx_count+1);
            Ok(())
        }

        pub fn withdraw(origin, vault_id: u32, amount: u64, sig: Signature) -> Result {
            let sender = ensure_signed(origin)?;
            ensure!(<PublicKeys<T>>::exists(sender.clone()), "Sender not registered");
            let tx_count = <BlockTransactions<T>>::get();
            ensure!(tx_count< SIZE_LIMIT, "Full block");

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
            <BlockTransactions<T>>::put(tx_count+1);
            Ok(())
        }

        fn on_finalize() {
            let vaults_hash = Self::hash_balance_tree();
            <finality::Module<T>>::add_hash(vaults_hash);
            <BlockTransactions<T>>::kill();
        }
    }
}

// Hash of the zero vault ie hash(0, 0)
pub const EMPTY_VAULT_HASH: [u8; 32] =
    hex!("049ee3eba8c1600700ee1b87eb599f16716b0b1022947733551fde4050ca6804");

impl<T: Trait> Module<T> {
    #[cfg(feature = "std")]
    fn vault_set_up(substrate_who: T::AccountId, which: u32, nonce: u32, vault: Vault) -> Result {
        let max_id = <RecycledID<T>>::get(0);
        <RecycledID<T>>::insert(0, max_id + 1);
        <PublicKeys<T>>::insert(substrate_who, vault.owner.clone());
        <Nonces<T>>::insert(vault.owner.clone(), nonce);
        <Vaults<T>>::insert(which, vault);
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
        for _ in 0..(depth - 1) {
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
    // TODO: Why does this need to be pub
    #[allow(unreachable_pub)]
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
        let paul_public: PublicKey = public_key(&paul_private.to_bytes_be()).into();
        let remco_public: PublicKey = public_key(&remco_private.to_bytes_be()).into();

        // Note - not realistic ids
        let eth_id: [u8; 24] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ];
        let dai_id: [u8; 24] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
        ];

        let paul_eth_vault = Vault {
            owner:    paul_public.clone(),
            token_id: eth_id,
            balance:  100,
        };
        let paul_dai_vault = Vault {
            owner:    paul_public.clone(),
            token_id: dai_id,
            balance:  1000,
        };
        let remco_eth_vault = Vault {
            owner:    remco_public.clone(),
            token_id: eth_id,
            balance:  100,
        };
        let remco_dai_vault = Vault {
            owner:    remco_public.clone(),
            token_id: dai_id,
            balance:  1000,
        };

        let mut t = system::GenesisConfig::<ExchangeTest>::default()
            .build_storage()
            .unwrap()
            .0;
        t.extend(
            GenesisConfig::<ExchangeTest> {
                // (owner substrate id, new vault id, nonce, vault)
                // Note that this method will reset the substrate account id system on each call.
                owners: vec![
                    (0, 0, 0, paul_eth_vault),
                    (0, 1, 0, paul_dai_vault),
                    (1, 2, 1, remco_eth_vault),
                    (1, 3, 1, remco_dai_vault),
                ],
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

    // Test of totally valid transaction
    #[test]
    fn valid_exchange() {
        let paul_private =
            u256h!("02c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let remco_private =
            u256h!("04c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");

        let eth_id: [u8; 24] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ];
        let dai_id: [u8; 24] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
        ];

        let mut paul_maker = MakerMessage {
            vault_a:  0,
            vault_b:  1,
            amount_a: 1,
            amount_b: 300,
            token_a:  eth_id,
            token_b:  dai_id,
            trade_id: 0,
            sig:      Signature {
                r: [0; 32],
                s: [0; 32],
            },
        };
        let hashed_maker = maker_hash(&paul_maker);
        let maker_sig: Signature = sign(&hashed_maker, &paul_private.to_bytes_be()).into();
        paul_maker.sig = maker_sig;

        let remco_taker = TakerMessage {
            maker_message: paul_maker,
            vault_a:       2,
            vault_b:       3,
        };
        let hashed_taker = taker_hash(&remco_taker);
        let taker_sig: Signature = sign(&hashed_taker, &remco_private.to_bytes_be()).into();

        with_externalities(&mut new_test_ext(), || {
            assert_ok!(Exchange::execute_order(
                Origin::signed(1),
                remco_taker,
                taker_sig
            ));
            assert_eq!(Exchange::get_vault(0).balance, 99);
            assert_eq!(Exchange::get_vault(1).balance, 1300);
            assert_eq!(Exchange::get_vault(2).balance, 101);
            assert_eq!(Exchange::get_vault(3).balance, 700);
            assert_eq!(
                Exchange::hash_balance_tree(),
                hex!("0192788d854aab1b0cbee2da24082981e021f51e0197828631a7b552355ab99e")
            );
        });
    }

    // Test of valid replayed transaction
    #[test]
    fn valid_replayed_transaction() {
        let paul_private =
            u256h!("02c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let remco_private =
            u256h!("04c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");

        let eth_id: [u8; 24] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ];
        let dai_id: [u8; 24] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
        ];

        let mut paul_maker = MakerMessage {
            vault_a:  0,
            vault_b:  1,
            amount_a: 1,
            amount_b: 300,
            token_a:  eth_id,
            token_b:  dai_id,
            trade_id: 0,
            sig:      Signature {
                r: [0; 32],
                s: [0; 32],
            },
        };
        let hashed_maker = maker_hash(&paul_maker);
        let maker_sig: Signature = sign(&hashed_maker, &paul_private.to_bytes_be()).into();
        paul_maker.sig = maker_sig;

        let remco_taker = TakerMessage {
            maker_message: paul_maker,
            vault_a:       2,
            vault_b:       3,
        };
        let hashed_taker = taker_hash(&remco_taker);
        let taker_sig: Signature = sign(&hashed_taker, &remco_private.to_bytes_be()).into();

        with_externalities(&mut new_test_ext(), || {
            assert_ok!(Exchange::execute_order(
                Origin::signed(1),
                remco_taker.clone(),
                taker_sig.clone()
            ));
            assert_eq!(
                Exchange::execute_order(Origin::signed(1), remco_taker, taker_sig),
                Err("Trade has already been executed")
            );
        });
    }

    // Test of totally valid withdraw
    #[test]
    fn valid_withdraw() {
        let remco_private =
            u256h!("04c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let remco_public: PublicKey = public_key(&remco_private.to_bytes_be()).into();

        let eth_id: [u8; 24] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ];
        let remco_eth_vault = Vault {
            owner:    remco_public.clone(),
            token_id: eth_id,
            balance:  100,
        };

        // In our init we gave the remco key a nonce of one, and now we are withdrawing
        // 10
        let hash = hash(
            ((U256::from(1_u64) << 64) + U256::from(10)).to_bytes_be(),
            remco_eth_vault.vault_hash(),
        );
        let sig: Signature = sign(&hash, &remco_private.to_bytes_be()).into();

        with_externalities(&mut new_test_ext(), || {
            assert_ok!(Exchange::withdraw(Origin::signed(1), 2, 10, sig));
            assert_eq!(Exchange::get_vault(2).balance, 90);
        });
    }
    // Test of invalid withdraw conditions
    #[test]
    fn invalid_withdraw() {
        let remco_private =
            u256h!("04c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let remco_public: PublicKey = public_key(&remco_private.to_bytes_be()).into();

        let eth_id: [u8; 24] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ];
        let remco_eth_vault = Vault {
            owner:    remco_public.clone(),
            token_id: eth_id,
            balance:  100,
        };

        // In our init we gave the remco key a nonce of one, and now we are withdrawing
        // 10
        let hash_1 = hash(
            ((U256::from(1_u64) << 64) + U256::from(10)).to_bytes_be(),
            remco_eth_vault.vault_hash(),
        );
        let sig_1: Signature = sign(&hash_1, &remco_private.to_bytes_be()).into();

        let remco_eth_vault = Vault {
            owner:    remco_public.clone(),
            token_id: eth_id,
            balance:  90,
        };
        let hash_2 = hash(
            ((U256::from(2_u64) << 64) + U256::from(101)).to_bytes_be(),
            remco_eth_vault.vault_hash(),
        );
        let sig_2: Signature = sign(&hash_2, &remco_private.to_bytes_be()).into();

        with_externalities(&mut new_test_ext(), || {
            assert_ok!(Exchange::withdraw(Origin::signed(1), 2, 10, sig_1.clone()));
            // Invalid signature because it signs a lower nonce than the system knows
            assert_eq!(
                Exchange::withdraw(Origin::signed(1), 2, 10, sig_1.clone()),
                Err("Invalid Signature")
            );
            assert_eq!(
                Exchange::withdraw(Origin::signed(1), 0, 10, sig_1),
                Err("Sender doesn't own the vault")
            );
            assert_eq!(
                Exchange::withdraw(Origin::signed(1), 2, 101, sig_2),
                Err("Trying to withdraw too much")
            );
        });
    }

    // Test of each invalid condition in transaction
    #[test]
    fn invalid_exchange() {
        let paul_private =
            u256h!("02c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let remco_private =
            u256h!("04c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");

        let eth_id: [u8; 24] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ];
        let dai_id: [u8; 24] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
        ];
        let zrx_id: [u8; 24] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3,
        ];

        let mut paul_maker = MakerMessage {
            vault_a:  5,
            vault_b:  5,
            amount_a: 1_000_000,
            amount_b: 100_000,
            token_a:  zrx_id,
            token_b:  zrx_id,
            trade_id: 0,
            sig:      Signature {
                r: [0; 32],
                s: [0; 32],
            },
        };
        let hashed_maker = maker_hash(&paul_maker);
        let maker_sig: Signature = sign(&hashed_maker, &paul_private.to_bytes_be()).into();
        paul_maker.sig = maker_sig;

        let mut remco_taker = TakerMessage {
            maker_message: paul_maker,
            vault_a:       5,
            vault_b:       5,
        };
        let hashed_taker = taker_hash(&remco_taker);
        let taker_sig: Signature = sign(&hashed_taker, &remco_private.to_bytes_be()).into();

        with_externalities(&mut new_test_ext(), || {
            assert_eq!(
                Exchange::execute_order(Origin::signed(1), remco_taker.clone(), taker_sig.clone()),
                Err("Missing taker vault a")
            );
            remco_taker.vault_a = 0;
            assert_eq!(
                Exchange::execute_order(Origin::signed(1), remco_taker.clone(), taker_sig.clone()),
                Err("Missing taker vault b")
            );
            remco_taker.vault_b = 3;
            assert_eq!(
                Exchange::execute_order(Origin::signed(1), remco_taker.clone(), taker_sig.clone()),
                Err("Missing maker vault a")
            );
            remco_taker.maker_message.vault_a = 1;
            assert_eq!(
                Exchange::execute_order(Origin::signed(1), remco_taker.clone(), taker_sig.clone()),
                Err("Missing maker vault b")
            );
            remco_taker.maker_message.vault_b = 2;

            assert_eq!(
                Exchange::execute_order(Origin::signed(1), remco_taker.clone(), taker_sig.clone()),
                Err("Mismatch in taker vault owners")
            );
            remco_taker.vault_a = 2;
            remco_taker.vault_b = 3;
            assert_eq!(
                Exchange::execute_order(Origin::signed(1), remco_taker.clone(), taker_sig.clone()),
                Err("Mismatch in maker vault owners")
            );
            remco_taker.maker_message.vault_a = 1;
            remco_taker.maker_message.vault_b = 0;

            assert_eq!(
                Exchange::execute_order(Origin::signed(1), remco_taker.clone(), taker_sig.clone()),
                Err("Token in maker order vault_a doesn't match vault token type")
            );
            remco_taker.maker_message.token_a = dai_id;
            assert_eq!(
                Exchange::execute_order(Origin::signed(1), remco_taker.clone(), taker_sig.clone()),
                Err("Token in maker order vault_b doesn't match vault token type")
            );
            remco_taker.maker_message.token_b = eth_id;

            assert_eq!(
                Exchange::execute_order(Origin::signed(1), remco_taker.clone(), taker_sig.clone()),
                Err("Mismatched token_a types")
            );
            remco_taker.maker_message.vault_a = 0;
            remco_taker.maker_message.token_a = eth_id;
            assert_eq!(
                Exchange::execute_order(Origin::signed(1), remco_taker.clone(), taker_sig.clone()),
                Err("Mismatched token_b types")
            );
            remco_taker.maker_message.vault_b = 1;
            remco_taker.maker_message.token_b = dai_id;

            assert_eq!(
                Exchange::execute_order(Origin::signed(1), remco_taker.clone(), taker_sig.clone()),
                Err("Not enough funds in maker's source vault")
            );
            remco_taker.maker_message.amount_a = 1;
            assert_eq!(
                Exchange::execute_order(Origin::signed(1), remco_taker.clone(), taker_sig.clone()),
                Err("Not enough funds in taker's source vault")
            );
            remco_taker.maker_message.amount_b = 300;

            assert_eq!(
                Exchange::execute_order(Origin::signed(1), remco_taker.clone(), taker_sig.clone()),
                Err("Maker message improperly signed")
            );
        });
    }

    // Test of emptying a vault and the id getting recycled
    #[test]
    fn id_recycling() {
        let paul_private =
            u256h!("02c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let remco_private =
            u256h!("04c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let paul_public: PublicKey = public_key(&paul_private.to_bytes_be()).into();

        let eth_id: [u8; 24] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ];
        let dai_id: [u8; 24] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
        ];
        let zrx_id: [u8; 24] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
        ];

        // This order will empty vault 0 for the maker and vault 3 for the taker
        let mut paul_maker = MakerMessage {
            vault_a:  0,
            vault_b:  1,
            amount_a: 100,
            amount_b: 1000,
            token_a:  eth_id,
            token_b:  dai_id,
            trade_id: 0,
            sig:      Signature {
                r: [0; 32],
                s: [0; 32],
            },
        };
        let hashed_maker = maker_hash(&paul_maker);
        let maker_sig: Signature = sign(&hashed_maker, &paul_private.to_bytes_be()).into();
        paul_maker.sig = maker_sig;

        let remco_taker = TakerMessage {
            maker_message: paul_maker,
            vault_a:       2,
            vault_b:       3,
        };
        let hashed_taker = taker_hash(&remco_taker);
        let taker_sig: Signature = sign(&hashed_taker, &remco_private.to_bytes_be()).into();

        let zero_vault = Vault {
            owner:    PublicKey {
                x: [0; 32],
                y: [0; 32],
            },
            token_id: [0; 24],
            balance:  0,
        };
        let mut test_vault = Vault {
            owner:    paul_public.clone(),
            token_id: eth_id,
            balance:  0,
        };

        with_externalities(&mut new_test_ext(), || {
            assert_ok!(Exchange::execute_order(
                Origin::signed(1),
                remco_taker,
                taker_sig
            ));
            // Check that the vaults are deleted
            assert_eq!(Exchange::get_vault(0), zero_vault.clone());
            assert_eq!(Exchange::get_vault(3), zero_vault);

            // Check that the index of the next vault released is 2
            assert_eq!(Exchange::get_available_id(), 2);

            // Check that the indexing contains each possible new vault.
            assert_eq!(Exchange::get_recycled_id(2), 3);
            assert_eq!(Exchange::get_recycled_id(1), 0);
            assert_eq!(Exchange::get_recycled_id(0), 4);

            // Check that a new vault is registerable and gets the right id.
            assert_ok!(Exchange::vault_registration(Origin::signed(0), eth_id));
            assert_ok!(Exchange::vault_registration(Origin::signed(0), dai_id));
            assert_ok!(Exchange::vault_registration(Origin::signed(0), zrx_id));

            // Checks that each vault is assigned to recycled ids then to properly
            // incrementing new ones.
            assert_eq!(Exchange::get_vault(3), test_vault.clone());
            test_vault.token_id = dai_id;
            assert_eq!(Exchange::get_vault(0), test_vault.clone());
            test_vault.token_id = zrx_id;
            assert_eq!(Exchange::get_vault(4), test_vault.clone());
            assert_eq!(Exchange::get_available_id(), 0);
            assert_eq!(Exchange::get_recycled_id(0), 5);
        });
    }
}
