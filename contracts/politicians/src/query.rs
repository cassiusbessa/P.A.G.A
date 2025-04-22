use cosmwasm_std::{Deps, StdResult, Addr};
use crate::state::{
    PROMISES, PROMISES_BY_POLITICIAN, POLITICIANS, VOTES_BY_ELECTOR,
};


pub fn get_promises_by_politician(deps: Deps, politician: String) -> StdResult<Vec<u64>> {
    let addr = deps.api.addr_validate(&politician)?;
    let promises = PROMISES_BY_POLITICIAN.may_load(deps.storage, &addr)?.unwrap_or_default();
    Ok(promises)
}

pub fn get_promise(deps: Deps, politician: String, promise_id: u64) -> StdResult<Option<crate::state::Promise>> {
    let addr = deps.api.addr_validate(&politician)?;
    let promise = PROMISES.may_load(deps.storage, (&addr, promise_id))?;
    Ok(promise)
}

pub fn get_votes_by_elector(deps: Deps, elector: String) -> StdResult<Vec<(Addr, u64)>> {
    let addr = deps.api.addr_validate(&elector)?;
    let votes = VOTES_BY_ELECTOR.may_load(deps.storage, &addr)?.unwrap_or_default();
    Ok(votes)
}

pub fn get_politician(deps: Deps, address: String) -> StdResult<Option<crate::state::Politician>> {
    let addr = deps.api.addr_validate(&address)?;
    let politician = POLITICIANS.may_load(deps.storage, &addr)?;
    Ok(politician)
}