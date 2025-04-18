use cosmwasm_std::{Deps, StdResult, Addr, to_json_binary, Binary};
use crate::state::ELECTORS;


pub fn query_elector(deps: Deps, address: Addr) -> StdResult<Binary> {

    let elector = ELECTORS.load(deps.storage, &address)?;
    to_json_binary(&elector)
}

pub fn query_balance(deps: Deps, address: Addr) -> StdResult<Binary> {

    let elector = ELECTORS.load(deps.storage, &address)?;
    to_json_binary(&elector.balance)
}
