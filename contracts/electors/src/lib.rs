mod execute;
mod state;
mod msg;
mod errors;
mod query;


use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult, Binary, Deps};
use crate::msg::{ExecuteMsg, QueryMsg, InstantiateMsg};
use crate::errors::ContractError;
use crate::execute::{execute_register, execute_follow};
use crate::state::OWNER;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let owner_addr = deps.api.addr_validate(&msg.owner)?;
    OWNER.save(deps.storage, &owner_addr)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", owner_addr))
}


#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register {} => execute_register(deps, env, info),
        ExecuteMsg::FollowPolitician { role, politician_address } => execute_follow(deps, env, info, role, politician_address),
        ExecuteMsg::AddBalance { amount } => {
            // Aqui você pode implementar a lógica para adicionar saldo
            // Por enquanto, vamos apenas retornar um erro
            Err(ContractError::NotImplemented {})
        }
    }
}

use crate::query::{query_elector, query_balance};

#[entry_point]
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

