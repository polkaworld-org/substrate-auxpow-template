use std::sync::Arc;

use client::blockchain::HeaderBackend;
use codec::Encode;
use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;
use node_primitives::{AccountId, AccountNonceApi, Block, BlockId, Index};
use sr_primitives::traits;
use substrate_primitives::hexdisplay::HexDisplay;
use transaction_pool::txpool::{self, Pool};

pub use self::gen_client::Client as AccountsClient;

const RUNTIME_ERROR: i64 = 1;

/// MergedMining RPC methods.
#[rpc]
pub trait MergedMiningApi {
    /// Generate an auxiliary block parameters.
    #[rpc(name = "createauxblock")]
    fn create_auxpow_block(&self) -> Result<Index>;

    /// Submit the solved auxpow of an auxiliary block.
    #[rpc(name = "submitauxblock")]
    fn submit_auxpow(&self, auxpow: String) -> Result<Index>;
}

/// An implementation of MergedMining specific RPC methods.
pub struct MergedMining<P: txpool::ChainApi, C> {
    client: Arc<C>,
    pool: Arc<Pool<P>>,
}

impl<P: txpool::ChainApi, C> MergedMining<P, C> {
    /// Create new `MergedMining` given client and transaction pool.
    pub fn new(client: Arc<C>, pool: Arc<Pool<P>>) -> Self {
        MergedMining { client, pool }
    }
}

impl<P, C> MergedMiningApi for MergedMining<P, C>
where
    C: traits::ProvideRuntimeApi,
    C: HeaderBackend<Block>,
    C: Send + Sync + 'static,
    P: txpool::ChainApi + Sync + Send + 'static,
{
    fn create_auxpow_block(&self) -> Result<Index> {
        Ok(0)
    }

    fn submit_auxpow(&self, auxpow: String) -> Result<Index> {
        println!("{:?}", auxpow);
        Ok(0)
    }
}
