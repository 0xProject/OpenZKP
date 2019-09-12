//! `Service` and `ServiceFactory` implementation. Specialized wrapper over
//! Substrate service.

#![warn(unused_extern_crates)]

use crate::assets::RUNTIME_WASM;
use basic_authorship::ProposerFactory;
use consensus::{import_queue, start_aura, AuraImportQueue, NothingExtra, SlotDuration};
use inherents::InherentDataProviders;
use log::info;
use network::construct_simple_protocol;
use primitives::{ed25519::Pair, Pair as PairT};
use std::sync::Arc;
use substrate_client as client;
use substrate_executor::native_executor_instance;
use substrate_runtime::{self, block_proof, opaque::Block, GenesisConfig, RuntimeApi};
use substrate_service::{
    construct_service_factory, FactoryFullConfiguration, FullBackend, FullClient, FullComponents,
    FullExecutor, LightBackend, LightClient, LightComponents, LightExecutor, TaskExecutor,
};
use transaction_pool::{self, txpool::Pool as TransactionPool};

// TODO: Remove: pub(crate) use substrate_executor::NativeExecutor;

// Our native executor instance.
native_executor_instance!(
	pub Executor,
	substrate_runtime::api::dispatch,
	substrate_runtime::native_version,
	RUNTIME_WASM
);

#[derive(Default)]
// TODO: Why does this need to be pub
#[allow(unreachable_pub)]
pub struct NodeConfig {
    inherent_data_providers: InherentDataProviders,
}

#[cfg(feature = "std")]
impl std::fmt::Debug for NodeConfig {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "NodeConfig(...)")
    }
}

construct_simple_protocol! {
    /// Demo protocol attachment for substrate.
    pub struct NodeProtocol where Block = Block { }
}

construct_service_factory! {
    struct Factory {
        Block = Block,
        RuntimeApi = RuntimeApi,
        NetworkProtocol = NodeProtocol { |config| Ok(NodeProtocol::new()) },
        RuntimeDispatch = Executor,
        FullTransactionPoolApi = transaction_pool::ChainApi<client::Client<FullBackend<Self>, FullExecutor<Self>, Block, RuntimeApi>, Block>
            { |config, client| Ok(TransactionPool::new(config, transaction_pool::ChainApi::new(client))) },
        LightTransactionPoolApi = transaction_pool::ChainApi<client::Client<LightBackend<Self>, LightExecutor<Self>, Block, RuntimeApi>, Block>
            { |config, client| Ok(TransactionPool::new(config, transaction_pool::ChainApi::new(client))) },
        Genesis = GenesisConfig,
        Configuration = NodeConfig,
        FullService = FullComponents<Self>
            { |config: FactoryFullConfiguration<Self>, executor: TaskExecutor|
                FullComponents::<Factory>::new(config, executor)
            },
        AuthoritySetup = {
            |service: Self::FullService, executor: TaskExecutor, key: Option<Arc<Pair>>| {
                if let Some(key) = key {
                    info!("Using authority key {}", key.public());
                    let proposer = Arc::new(ProposerFactory {
                        client: service.client(),
                        transaction_pool: service.transaction_pool(),
                        inherents_pool: service.inherents_pool(),
                    });
                    let client = service.client();
                    let reg = service.config.custom.inherent_data_providers.register_provider(block_proof::InherentDataProvider);
                    executor.spawn(start_aura(
                        SlotDuration::get_or_compute(&*client)?,
                        key.clone(),
                        client.clone(),
                        client,
                        proposer,
                        service.network(),
                        service.on_exit(),
                        service.config.custom.inherent_data_providers.clone(),
                        service.config.force_authoring,
                    )?);
                }

                Ok(service)
            }
        },
        LightService = LightComponents<Self>
            { |config, executor| <LightComponents<Factory>>::new(config, executor) },
        FullImportQueue = AuraImportQueue<
            Self::Block,
        >
            { |config: &mut FactoryFullConfiguration<Self> , client: Arc<FullClient<Self>>| {
                    import_queue::<_, _, _, Pair>(
                        SlotDuration::get_or_compute(&*client)?,
                        client.clone(),
                        None,
                        client,
                        NothingExtra,
                        config.custom.inherent_data_providers.clone(),
                    ).map_err(Into::into)
                }
            },
        LightImportQueue = AuraImportQueue<
            Self::Block,
        >
            { |config: &mut FactoryFullConfiguration<Self>, client: Arc<LightClient<Self>>| {
                    import_queue::<_, _, _, Pair>(
                        SlotDuration::get_or_compute(&*client)?,
                        client.clone(),
                        None,
                        client,
                        NothingExtra,
                        config.custom.inherent_data_providers.clone(),
                    ).map_err(Into::into)
                }
            },
    }
}
