use namada::tendermint_rpc::HttpClient;
use namada::types::control_flow::Halt;
use namada::types::io::Io;
use namada_sdk::queries::Client;
use namada_sdk::rpc::wait_until_node_is_synched;
use tendermint_config::net::Address as TendermintAddress;

use crate::client::utils;

/// Trait for clients that can be used with the CLI.
#[async_trait::async_trait(?Send)]
pub trait CliClient: Client + Sync {
    fn from_tendermint_address(address: &mut TendermintAddress) -> Self;
    async fn wait_until_node_is_synced(&self, io: &impl Io) -> Halt<()>;
}

#[async_trait::async_trait(?Send)]
impl CliClient for HttpClient {
    fn from_tendermint_address(address: &mut TendermintAddress) -> Self {
        HttpClient::new(utils::take_config_address(address)).unwrap()
    }

    async fn wait_until_node_is_synced(&self, io: &impl Io) -> Halt<()> {
        wait_until_node_is_synched(self, io).await
    }
}

pub struct CliIo;

#[async_trait::async_trait(?Send)]
impl Io for CliIo {}

pub struct CliApi;
