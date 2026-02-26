use std::str::FromStr;

use async_trait::async_trait;
use bitcoin::Network;
use ldk_server_client::client::LdkServerClient;
use lightning::ln::features::NodeFeatures;

use crate::{LightningError, LightningNode, NodeInfo};

pub struct LdkServerConnection {
    pub base_url: String,
    pub api_key: String,
    pub server_cert_pem: Vec<u8>,
}

pub struct LdkServer {
    client: LdkServerClient,
    info: NodeInfo,
    network: Network,
}

impl LdkServer {
    /// Creates a new instance that connects to a running `ldk-server` node (daemon).
    pub async fn new(connection: LdkServerConnection) -> Result<Self, LightningError> {
        let client = LdkServerClient::new(base_url, api_key, server_cert_pem)
            .map_err(|e| LightningError::ConnectionError(e))?;

        let node_info_res = client
            .get_node_info(node_info_req)
            .await
            .map_err(|e| LightningError::GetNodeInfoError(e.to_string()))?;

        let network = Network::from_str(node_info_res.network.as_str())
            .map_err(|e| LightningError::GetNodeInfoError(e.to_string()))?;

        let info = NodeInfo {
            pubkey: node_info_res.node_id.into(),
            alias: node_info_res.node_alias().to_string(),
            features: match node_info_res.features {
                Some(features) => NodeFeatures::from_be_bytes(features.node.to_vec()),
                None => NodeFeatures::empty(),
            },
        };

        Ok(Self {
            client,
            info,
            network,
        })
    }
}

#[async_trait]
impl LightningNode for LdkServer {
    fn get_info(&self) -> &NodeInfo {
        todo!()
    }

    fn get_network(&self) -> Network {
        todo!()
    }

    async fn send_payment(
        &self,
        dest: PublicKey,
        amount_msat: u64,
    ) -> Result<PaymentHash, LightningError> {
        todo!()
    }

    async fn track_payment(
        &self,
        hash: &PaymentHash,
        shutdown: Listener,
    ) -> Result<PaymentResult, LightningError> {
        todo!()
    }

    async fn get_node_info(&self, node_id: &PublicKey) -> Result<NodeInfo, LightningError> {
        todo!()
    }

    async fn channel_capacities(&self) -> Result<u64, LightningError> {
        todo!()
    }

    async fn get_graph(&self) -> Result<Graph, LightningError> {
        todo!()
    }
}
