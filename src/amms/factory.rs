use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};

use alloy::{
    network::Network,
    primitives::{Address, B256},
    providers::Provider,
    rpc::types::eth::{Filter, Log},
    sol_types::SolEvent,
    transports::Transport,
};
use futures::stream::{FuturesUnordered, StreamExt};
use serde::{Deserialize, Serialize};

use super::error::AMMError;

//TODO: add consts for steps, batch size, etc.
pub trait AutomatedMarketMakerFactory {
    //TODO: GAT for AMM

    /// Returns the address of the factory.
    fn address(&self) -> Address;

    // TODO: event sig

    /// Returns the block number at which the factory was created.
    fn creation_block(&self) -> u64;
}

macro_rules! factory {
    ($($factory_type:ident),+ $(,)?) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum Factory {
            $($factory_type($factory_type),)+
        }

        impl AutomatedMarketMakerFactory for Factory {
            fn address(&self) -> Address {
                match self {
                    $(Factory::$factory_type(factory) => factory.address(),)+
                }
            }

            // TODO: event sig

            fn creation_block(&self) -> u64 {
                match self {
                    $(Factory::$factory_type(factory) => factory.creation_block(),)+
                }
            }
        }

        impl Hash for Factory {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.address().hash(state);
            }
        }

        impl PartialEq for Factory {
            fn eq(&self, other: &Self) -> bool {
                self.address() == other.address()
            }
        }

        impl Eq for Factory {}
    };
}

// factory!(UniswapV2Factory);
