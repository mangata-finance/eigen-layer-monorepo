use std::sync::Arc;

use bindings::{
    delegation_manager::DelegationManager, erc20_mock::ERC20Mock, i_strategy::IStrategy,
    registry_coordinator::RegistryCoordinator, stake_registry::StakeRegistry,
    strategy_manager::StrategyManager,
};
use ethers::{
    contract::{ContractError, EthError},
    middleware::{MiddlewareBuilder, NonceManagerMiddleware, SignerMiddleware},
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
    utils::parse_ether,
};
use eyre::eyre;
use sp_core::hexdisplay::AsBytesRef;
use tracing::{debug, info, instrument};

use crate::cli::CliArgs;

pub mod avs;
pub mod eigen;

type MW = Provider<Http>;
pub type Client = SignerMiddleware<NonceManagerMiddleware<MW>, LocalWallet>;

#[instrument(skip_all)]
pub(crate) async fn build_eth_client(cfg: &CliArgs) -> eyre::Result<Client> {
    let provider: Provider<Http> = MW::try_from(cfg.eth_rpc_url.clone())?;
    info!("Eth Wallet decryting...");
    let wallet = cfg.get_ecdsa_keystore()?.into_wallet()?;
    info!("Eth Wallet decrytped with address {:x}", wallet.address());
    let nonce = NonceManagerMiddleware::new(provider, wallet.address());
    let client = Client::new_with_provider_chain(nonce, wallet.with_chain_id(cfg.chain_id)).await?;

    Ok(client)
}

pub(crate) fn map_revert(e: ContractError<Client>) -> eyre::Report {
    match e {
        ContractError::Revert(b) => eyre!(
            "Contract call reverted with message: {}",
            String::decode_with_selector(b.as_bytes_ref())
                .unwrap_or("cannot parse message".to_string())
        ),
        _ => eyre::Report::new(e),
    }
}

#[instrument(skip_all)]
pub(crate) async fn setup_deposits(
    eth_rpc_url: String,
    registry_address: Address,
    stake: u32,
    operator: LocalWallet,
) -> eyre::Result<()> {
    let op_address = operator.address();
    anvil_balance(eth_rpc_url.clone(), op_address, 100).await?;
    debug!("set some ether to operator");

    let provider: Provider<Http> = MW::try_from(eth_rpc_url)?;
    let client = Arc::new(provider.with_signer(operator));
    let registry = RegistryCoordinator::new(registry_address, client.clone());
    let stake_registry_address = registry.stake_registry().await?;
    let stake_reg = StakeRegistry::new(stake_registry_address, client.clone());
    let delegation_address = stake_reg.delegation().await?;
    let delegation = DelegationManager::new(delegation_address, client.clone());
    let strategy_manager_address = delegation.strategy_manager().await?;
    let strategy_params = stake_reg.strategy_params_by_index(0, 0.into()).await?;
    let strategy_manager = StrategyManager::new(strategy_manager_address, client.clone());
    debug!("startegy address {:?}", strategy_params);
    let strategy = IStrategy::new(strategy_params.strategy, client.clone());
    let erc20_address = strategy.underlying_token().call().await?;
    debug!("erc address {:?}", erc20_address);

    let erc20 = ERC20Mock::new(erc20_address, client.clone());
    erc20.mint(op_address, stake.into()).send().await?.await?;
    debug!("sent some erc20 to operator");
    erc20
        .approve(strategy_manager_address, stake.into())
        .send()
        .await?
        .await?;
    debug!("approve startegy manager for erc20 for operator");
    strategy_manager
        .deposit_into_strategy(strategy_params.strategy, erc20_address, stake.into())
        .send()
        .await?
        .await?;
    debug!("deposited into startegy manager for erc20 for operator");
    Ok(())
}

async fn anvil_balance(
    eth_rpc_url: String,
    address: Address,
    ether: u128,
) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .post(eth_rpc_url)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "method": "anvil_setBalance",
            "params": [address, parse_ether(ether).unwrap()],
            "id": 1
        }))
        .send()
        .await?;
    Ok(())
}
