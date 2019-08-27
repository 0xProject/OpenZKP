//! The Substrate Node Template runtime. This can be compiled with `#[no_std]`,
//! ready for Wasm.

#![cfg_attr(not(feature = "std"), no_std)]
// False positive: linter warns that the feature is stable and does not need
// a flag. But we do want to feature-flag it here.
#![allow(stable_features)] // TODO: Make it apply only to the following line
#![cfg_attr(not(feature = "std"), feature(alloc))]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]
// Substrate needs a large enum but we can't put this directly on it's declaration inside the
// substrate macro
#![allow(clippy::large_enum_variant)]

use client::{
    block_builder::api::{self as block_builder_api, CheckInherentsResult, InherentData},
    impl_runtime_apis, runtime_api,
};
use parity_codec::{Decode, Encode};
#[cfg(feature = "std")]
use primitives::bytes;
use primitives::{ed25519, sr25519, OpaqueMetadata};
use rstd::prelude::*;
use runtime_primitives::{
    create_runtime_str, generic,
    traits::{self, BlakeTwo256, Block as BlockT, NumberFor, StaticLookup, Verify},
    transaction_validity::TransactionValidity,
    ApplyResult,
};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
use version::NativeVersion;
use version::RuntimeVersion;
mod wrappers;

// A few exports that help ease life for downstream crates.
pub use balances::Call as BalancesCall;
pub use consensus::Call as ConsensusCall;
#[cfg(any(feature = "std", test))]
pub use runtime_primitives::BuildStorage;
pub use runtime_primitives::{Perbill, Permill};
pub use support::{construct_runtime, StorageValue};
pub use timestamp::{BlockPeriod, Call as TimestampCall};

/// The type that is used for identifying authorities.
pub type AuthorityId = <AuthoritySignature as Verify>::Signer;

/// The type used by authorities to prove their ID.
pub type AuthoritySignature = ed25519::Signature;

/// Alias to pubkey that identifies an account on the chain.
pub type AccountId = <AccountSignature as Verify>::Signer;

/// The type used by authorities to prove their ID.
pub type AccountSignature = sr25519::Signature;

/// A hash of some data used by the chain.
pub type Hash = primitives::H256;

/// Index of a block number in the chain.
pub type BlockNumber = u64;

/// Index of an account's extrinsic in the chain.
pub type Nonce = u64;

pub mod block_proof;
pub mod exchange;

/// Opaque types. These are used by the CLI to instantiate machinery that don't
/// need to know the specifics of the runtime. They can then be made to be
/// agnostic over specific formats of data like extrinsics, allowing for them to
/// continue syncing the network through upgrades to even the core
/// datastructures.
pub mod opaque {
    use super::*;

    /// Opaque, encoded, unchecked extrinsic.
    #[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub struct UncheckedExtrinsic(#[cfg_attr(feature = "std", serde(with = "bytes"))] pub Vec<u8>);
    #[cfg(feature = "std")]
    impl std::fmt::Debug for UncheckedExtrinsic {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(fmt, "{}", primitives::hexdisplay::HexDisplay::from(&self.0))
        }
    }
    impl traits::Extrinsic for UncheckedExtrinsic {
        fn is_signed(&self) -> Option<bool> {
            None
        }
    }
    /// Opaque block header type.
    pub type Header = generic::Header<
        BlockNumber,
        BlakeTwo256,
        generic::DigestItem<Hash, AuthorityId, AuthoritySignature>,
    >;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;
    /// Opaque session key type.
    pub type SessionKey = AuthorityId;
}

/// This runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name:         create_runtime_str!("substrate-node"),
    impl_name:         create_runtime_str!("substrate-node"),
    authoring_version: 3,
    spec_version:      4,
    impl_version:      4,
    apis:              RUNTIME_API_VERSIONS,
};

/// The version infromation used to identify this runtime when compiled
/// natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

impl system::Trait for Runtime {
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The index type for blocks.
    type BlockNumber = BlockNumber;
    /// The header digest type.
    type Digest = generic::Digest<Log>;
    /// The ubiquitous event type.
    type Event = Event;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The header type.
    type Header = generic::Header<BlockNumber, BlakeTwo256, Log>;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Nonce;
    /// The ubiquitous log type.
    type Log = Log;
    /// The lookup mechanism to get account ID from whatever is passed in
    /// dispatchers.
    type Lookup = Indices;
    /// The ubiquitous origin type.
    type Origin = Origin;
}

impl aura::Trait for Runtime {
    type HandleReport = ();
}

impl consensus::Trait for Runtime {
    // The aura module handles offline-reports internally
    // rather than using an explicit report system.
    type InherentOfflineReport = ();
    /// The ubiquitous log type.
    type Log = Log;
    /// The identifier we use to refer to authorities.
    type SessionKey = AuthorityId;
}

impl indices::Trait for Runtime {
    /// The type for recording indexing into the account enumeration. If this
    /// ever overflows, there will be problems!
    type AccountIndex = u32;
    /// The uniquitous event type.
    type Event = Event;
    /// Determine whether an account is dead.
    type IsDeadAccount = Balances;
    /// Use the standard means of resolving an index hint from an id.
    type ResolveHint = indices::SimpleResolveHint<Self::AccountId, Self::AccountIndex>;
}

impl timestamp::Trait for Runtime {
    /// A timestamp: seconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Aura;
}

impl balances::Trait for Runtime {
    /// The type for recording an account's balance.
    type Balance = u128;
    type DustRemoval = ();
    /// The uniquitous event type.
    type Event = Event;
    /// What to do if an account's free balance gets zeroed.
    type OnFreeBalanceZero = ();
    /// What to do if a new account is created.
    type OnNewAccount = Indices;
    type TransactionPayment = ();
    type TransferPayment = ();
}

impl sudo::Trait for Runtime {
    /// The uniquitous event type.
    type Event = Event;
    type Proposal = Call;
}

impl block_proof::Trait for Runtime {}

impl exchange::Trait for Runtime {
    type Event = Event;
}

construct_runtime!(
	pub enum Runtime with Log(InternalLog: DigestItem<Hash, AuthorityId, AuthoritySignature>) where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: system::{default, Log(ChangesTrieRoot)},
		Timestamp: timestamp::{Module, Call, Storage, Config<T>, Inherent},
		Consensus: consensus::{Module, Call, Storage, Config<T>, Log(AuthoritiesChange), Inherent},
		Aura: aura::{Module},
		Indices: indices,
		Balances: balances,
		Sudo: sudo,
        BlockProof: block_proof::{Module, Call, Storage, Inherent},
        Exchange: exchange::{Module, Call, Storage, Event<T>},
	}
);

/// The type used as a helper for interpreting the sender of transactions.
type Context = system::ChainContext<Runtime>;
/// The address format for describing accounts.
type Address = <Indices as StaticLookup>::Source;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256, Log>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
    generic::UncheckedMortalCompactExtrinsic<Address, Nonce, Call, AccountSignature>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Nonce, Call>;
/// Executive: handles dispatch to the various modules.
pub type Executive = executive::Executive<Runtime, Block, Context, Balances, AllModules>;

// Implement our runtime API endpoints. This is just a bunch of proxying.
impl_runtime_apis! {
    impl runtime_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block)
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header)
        }

        fn authorities() -> Vec<AuthorityId> {
            panic!("Deprecated, please use `AuthoritiesApi`.")
        }
    }

    impl runtime_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            Runtime::metadata().into()
        }
    }

    impl block_builder_api::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(block: Block, data: InherentData) -> CheckInherentsResult {
            data.check_extrinsics(&block)
        }

        fn random_seed() -> <Block as BlockT>::Hash {
            System::random_seed()
        }
    }

    impl runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(tx: <Block as BlockT>::Extrinsic) -> TransactionValidity {
            Executive::validate_transaction(tx)
        }
    }

    impl consensus_aura::AuraApi<Block> for Runtime {
        fn slot_duration() -> u64 {
            Aura::slot_duration()
        }
    }

    impl offchain_primitives::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(n: NumberFor<Block>) {
            Executive::offchain_worker(n)
        }
    }

    impl consensus_authorities::AuthoritiesApi<Block> for Runtime {
        fn authorities() -> Vec<AuthorityId> {
            Consensus::authorities()
        }
    }
}
