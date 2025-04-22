pub mod msg;
pub mod state;
pub mod utils;
pub mod execute;
pub mod errors;

use cosmwasm_std::{
    entry_point, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::msg::{InstantiateMsg, ExecuteMsg};
use crate::state::OWNER;
use crate::execute::{execute_update_electors_contract, execute_register_elector};
use crate::errors::ContractError;



#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let owner = deps.api.addr_validate(&msg.owner)?;
    OWNER.save(deps.storage, &owner)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", owner))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateElectorsContract { new_contract } => {
            execute_update_electors_contract(deps, info, new_contract)
        }
        ExecuteMsg::RegisterElector { } => execute_register_elector(deps, env, info)
    }
}
