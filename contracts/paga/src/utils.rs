use cosmwasm_std::{Deps, MessageInfo, StdError, StdResult};
use crate::state::OWNER;

pub fn only_owner(deps: Deps, info: &MessageInfo) -> StdResult<()> {
    let owner = OWNER.load(deps.storage)?;
    if info.sender != owner {
        return Err(StdError::generic_err("Unauthorized: not the contract owner"));
    }
    Ok(())
}
