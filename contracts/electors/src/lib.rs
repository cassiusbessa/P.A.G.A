extern crate alloc;

mod execute;
mod state;
pub mod msg;
mod errors;
mod query;
mod utils;


use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult, Binary, Deps};
use state::PAGA_CONTRACT;
use crate::msg::{ExecuteMsg, QueryMsg, InstantiateMsg};
use crate::errors::ContractError;
use crate::execute::{execute_register, execute_follow};
use crate::state::OWNER;



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let owner_addr = deps.api.addr_validate(&msg.owner)?;
    OWNER.save(deps.storage, &owner_addr)?;
    let paga_contract = deps.api.addr_validate(&msg.paga_contract)?;
    PAGA_CONTRACT.save(deps.storage, &paga_contract)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", owner_addr))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register { elector_address } =>  execute_register(deps, env, info, elector_address),

        ExecuteMsg::FollowPolitician { elector_address, role, politician_address } => execute_follow(deps, env, info, elector_address, role, politician_address),
        ExecuteMsg::AddBalance { amount } => {
            Err(ContractError::NotImplemented {})
        }
    }
}

use crate::query::{query_elector, query_balance};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetElector { address } => query_elector(deps, address),
        QueryMsg::GetBalance { address } => query_balance(deps, address),
    }
}

