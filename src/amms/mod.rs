pub mod error;
pub mod uniswap_v2;

use alloy::{
    network::Network,
    primitives::{Address, B256, U256},
    providers::Provider,
    transports::Transport,
};
use error::AMMError;
use serde::{Deserialize, Serialize};
use std::{
    future::Future,
    hash::{Hash, Hasher},
    sync::Arc,
};

use uniswap_v2::UniswapV2Pool;

pub trait AutomatedMarketMaker {
    // TODO: maybe add a sync step and batch size GAT that will be implemented for each amm

    /// Returns the address of the AMM.
    fn address(&self) -> Address;

    // TODO: need some way to keep in sync, maybe sync from log, maybe more elegant
    fn sync<T, N, P>(
        &mut self,
        provider: Arc<P>,
    ) -> impl Future<Output = Result<(), AMMError>> + Send
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>;

    // TODO: rename or rethink
    // NOTE: we should rethink how we are handling event signatures.
    // Ideally, the state space manager is able to know what it needs for discovery and sync signatures (discovery related to the factory). Revisit
    // maybe there is a way to have a specific action happen on a signature, implementing a type for each sig, just initial thoughts atm
    // TODO:
    fn sync_signatures(&self) -> Vec<B256>;

    /// Returns a vector of tokens in the AMM.
    fn tokens(&self) -> Vec<Address>;

    /// Calculates a f64 representation of base token price in the AMM.
    fn calculate_price(&self, base_token: Address, quote_token: Address) -> Result<f64, AMMError>;

    /// Locally simulates a swap in the AMM.
    /// Returns the amount received for `amount_in` of `token_in`.
    fn simulate_swap(
        &self,
        base_token: Address,
        quote_token: Address,
        amount_in: U256,
    ) -> Result<U256, AMMError>;

    /// Locally simulates a swap in the AMM.
    /// Mutates the AMM state to the state of the AMM after swapping.
    /// Returns the amount received for `amount_in` of `token_in`.
    fn simulate_swap_mut(
        &mut self,
        base_token: Address,
        quote_token: Address,
        amount_in: U256,
    ) -> Result<U256, AMMError>;
}

macro_rules! amm {
    ($($pool_type:ident),+ $(,)?) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum AMM {
            $($pool_type($pool_type),)+
        }

        impl AutomatedMarketMaker for AMM {
            fn address(&self) -> Address{
                match self {
                    $(AMM::$pool_type(pool) => pool.address(),)+
                }
            }

            async fn sync<T, N, P>(&mut self, middleware: Arc<P>) -> Result<(), AMMError>
            where
                T: Transport + Clone,
                N: Network,
                P: Provider<T, N>,
            {
                match self {
                    $(AMM::$pool_type(pool) => pool.sync(middleware).await,)+
                }
            }

            fn sync_signatures(&self) -> Vec<B256> {
                match self {
                    $(AMM::$pool_type(pool) => pool.sync_signatures(),)+
                }
            }

            fn simulate_swap(&self, base_token: Address, quote_token: Address,amount_in: U256) -> Result<U256, AMMError> {
                match self {
                    $(AMM::$pool_type(pool) => pool.simulate_swap(base_token, quote_token, amount_in),)+
                }
            }

            fn simulate_swap_mut(&mut self, base_token: Address, quote_token: Address, amount_in: U256) -> Result<U256, AMMError> {
                match self {
                    $(AMM::$pool_type(pool) => pool.simulate_swap_mut(base_token, quote_token, amount_in),)+
                }
            }

            fn tokens(&self) -> Vec<Address> {
                match self {
                    $(AMM::$pool_type(pool) => pool.tokens(),)+
                }
            }

            fn calculate_price(&self, base_token: Address, quote_token: Address) -> Result<f64, AMMError> {
                match self {
                    $(AMM::$pool_type(pool) => pool.calculate_price(base_token, quote_token),)+
                }
            }
        }

        impl Hash for AMM {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.address().hash(state);
            }
        }

        impl PartialEq for AMM {
            fn eq(&self, other: &Self) -> bool {
                self.address() == other.address()
            }
        }

        impl Eq for AMM {}
    };
}

amm!(UniswapV2Pool);
