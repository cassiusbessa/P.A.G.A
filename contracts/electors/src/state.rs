use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

pub const OWNER: Item<Addr> = Item::new("owner");
pub const PAGA_CONTRACT: Item<Addr> = Item::new("paga_contract");


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Elector {
    pub address: Addr,
    pub balance: u128,
    pub follows: Follows,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Follows {
    pub vereador: Option<Addr>,
    pub deputado_estadual: Option<Addr>,
    pub governador: Option<Addr>,
    pub deputado_federal: Option<Addr>,
    pub senador: Option<Addr>,
    pub presidente: Option<Addr>,
}

// Armazena todos os eleitores. A chave é o próprio endereço.
pub const ELECTORS: Map<&Addr, Elector> = Map::new("electors");
