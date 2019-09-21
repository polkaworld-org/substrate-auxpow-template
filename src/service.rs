#![warn(unused_extern_crates)]

//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use basic_authorship::ProposerFactory;
use codec::Encode;
use futures::prelude::*;
use inherents::InherentDataProviders;
use network::{config::DummyFinalityProofRequestBuilder, construct_simple_protocol};
use node_template_runtime::{
	self, opaque::Block, AccountId, GenesisConfig, RuntimeApi, WASM_BINARY,
};
use pow::{import_queue, start_mine, PowImportQueue, PowRuntimeAlgorithm};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use substrate_client::{self as client, LongestChain};
use substrate_executor::native_executor_instance;
pub use substrate_executor::NativeExecutor;
use substrate_service::{construct_service_factory, ServiceFactory, TelemetryOnConnect};
use substrate_service::{
	error::Error as ServiceError, FactoryFullConfiguration, FullBackend, FullClient,
	FullComponents, FullExecutor, LightBackend, LightClient, LightComponents, LightExecutor,
};
use transaction_pool::{self, txpool::Pool as TransactionPool};

// Our native executor instance.
native_executor_instance!(
	pub Executor,
	node_template_runtime::api::dispatch,
	node_template_runtime::native_version,
	WASM_BINARY
);

construct_simple_protocol! {
	/// Demo protocol attachment for substrate.
	pub struct NodeProtocol where Block = Block { }
}

pub struct NodeConfig {
	/// Tasks that were created by previous setup steps and should be spawned.
	pub tasks_to_spawn: Option<Vec<Box<dyn Future<Item = (), Error = ()> + Send>>>,
	inherent_data_providers: InherentDataProviders,
}

impl Default for NodeConfig {
	fn default() -> NodeConfig {
		NodeConfig {
			inherent_data_providers: InherentDataProviders::new(),
			tasks_to_spawn: None,
		}
	}
}

construct_service_factory! {
	struct Factory {
		Block = Block,
		RuntimeApi = RuntimeApi,
		NetworkProtocol = NodeProtocol { |config| Ok(NodeProtocol::new()) },
		RuntimeDispatch = Executor,
		FullTransactionPoolApi =
			transaction_pool::ChainApi<
				client::Client<FullBackend<Self>, FullExecutor<Self>, Block, RuntimeApi>,
				Block
			> {
				|config, client|
					Ok(TransactionPool::new(config, transaction_pool::ChainApi::new(client)))
			},
		LightTransactionPoolApi =
			transaction_pool::ChainApi<
				client::Client<LightBackend<Self>, LightExecutor<Self>, Block, RuntimeApi>,
				Block
			> {
				|config, client|
					Ok(TransactionPool::new(config, transaction_pool::ChainApi::new(client)))
			},
		Genesis = GenesisConfig,
		Configuration = NodeConfig,
		FullService = FullComponents<Self> {
			|config: FactoryFullConfiguration<Self>| FullComponents::<Factory>::new(config)
		},
		AuthoritySetup = {
			|mut service: Self::FullService| {
				if service.config().roles.is_authority() {
					let proposer = ProposerFactory {
						client: service.client(),
						transaction_pool: service.transaction_pool(),
					};

					start_mine(
						Box::new(service.client().clone()),
						service.client(),
						PowRuntimeAlgorithm::new(service.client()),
						proposer,
						crate::chain_spec::get_from_seed::<AccountId>("Alice").encode(),
						500,
						service.config().custom.inherent_data_providers.clone(),
					);
				}

				Ok(service)
			}
		},
		LightService = LightComponents<Self>
			{ |config| <LightComponents<Factory>>::new(config) },
		FullImportQueue = PowImportQueue<Self::Block> {
			|
				config: &mut FactoryFullConfiguration<Self>,
				client: Arc<FullClient<Self>>,
				select_chain: Self::SelectChain,
				transaction_pool: Option<Arc<TransactionPool<Self::FullTransactionPoolApi>>>,
			| {
				import_queue(
					Box::new(client.clone()),
					client.clone(),
					PowRuntimeAlgorithm::new(client.clone()),
					config.custom.inherent_data_providers.clone(),
				).map_err(Into::into)
			}
		},
		LightImportQueue = PowImportQueue<Self::Block>
			{ |config: &FactoryFullConfiguration<Self>, client: Arc<LightClient<Self>>| {
				let fprb = Box::new(DummyFinalityProofRequestBuilder::default()) as Box<_>;
				import_queue(
					Box::new(client.clone()),
					client.clone(),
					PowRuntimeAlgorithm::new(client.clone()),
					config.custom.inherent_data_providers.clone(),
				).map(|q| (q, fprb)).map_err(Into::into)
			}},
		SelectChain = LongestChain<FullBackend<Self>, Self::Block>
			{ |config: &FactoryFullConfiguration<Self>, client: Arc<FullClient<Self>>| {
				#[allow(deprecated)]
				Ok(LongestChain::new(client.backend().clone()))
			}
		},
		FinalityProofProvider = { |client: Arc<FullClient<Self>>| {
			Ok(None)
		}},
		RpcExtensions = jsonrpc_core::IoHandler<substrate_rpc::Metadata>
		{ |client, pool| {
			use auxpow_rpc::merged_mining::{MergedMining, MergedMiningApi};

			let mut io = jsonrpc_core::IoHandler::default();
			io.extend_with(
				MergedMiningApi::to_delegate(MergedMining::new(client, pool))
			);
			io
		}},
	}
}
