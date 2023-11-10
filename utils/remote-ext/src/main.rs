// TODO #167: fix clippy warnings
#![allow(clippy::all)]

#[macro_use]
extern crate log;

use clap::Parser;
use frame_remote_externalities::{Builder, Mode, OfflineConfig, OnlineConfig, RemoteExternalities};
use frame_support::traits::OnRuntimeUpgrade;
use jsonrpsee::ws_client::{WsClient, WsClientBuilder};
use sp_runtime::{traits::Block as BlockT, DeserializeOwned};

use anyhow::Result as AnyResult;
use sora2_parachain_runtime::Runtime;
use std::sync::Arc;

async fn create_ext<B>(client: Arc<WsClient>) -> AnyResult<RemoteExternalities<B>>
where
    B: DeserializeOwned + BlockT,
    <B as BlockT>::Header: DeserializeOwned,
{
    let res = Builder::<B>::new()
        .mode(Mode::OfflineOrElseOnline(
            OfflineConfig { state_snapshot: "state_snapshot".to_string().into() },
            OnlineConfig {
                transport: client.into(),
                state_snapshot: Some("state_snapshot".to_string().into()),
                ..Default::default()
            },
        ))
        .build()
        .await
        .unwrap();
    Ok(res)
}

#[derive(Debug, Clone, Parser)]
struct Cli {
    /// Sora node endpoint.
    uri: String,
}

#[tokio::main]
async fn main() -> AnyResult<()> {
    env_logger::init();
    let cli = Cli::parse();
    let client = WsClientBuilder::default()
        .max_request_body_size(u32::MAX)
        .build(cli.uri)
        .await?;
    let client = Arc::new(client);
    let mut ext = create_ext::<sora2_parachain_runtime::Block>(client.clone()).await?;
    let _res: AnyResult<()> = ext.execute_with(|| {
        for (account, data) in frame_system::Account::<Runtime>::iter() {
            info!("account: {:?}, data: {:?}", account, data);
        }
        info!("Start migrations");
        sora2_parachain_runtime::migrations::Migrations::on_runtime_upgrade();
        info!("Finish migrations");
        for (account, data) in frame_system::Account::<Runtime>::iter() {
            info!("account: {:?}, data: {:?}", account, data);
        }
        Ok(())
    });
    Ok(())
}
