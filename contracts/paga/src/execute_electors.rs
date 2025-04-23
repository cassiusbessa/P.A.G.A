use crate::{state::ELECTORS_CONTRACT, utils::load_politician};
use crate::utils::only_owner;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, WasmMsg, to_json_binary};
use serde_json::json;
use crate::errors::ContractError;



pub fn execute_update_electors_contract(
    deps: DepsMut,
    info: MessageInfo,
    new_contract: String,
) -> Result<Response, ContractError> {
    only_owner(deps.as_ref(), &info)?;

    let new_addr = deps.api.addr_validate(&new_contract)?;
    ELECTORS_CONTRACT.save(deps.storage, &new_addr)?;
    Ok(Response::new()
        .add_attribute("action", "update_electors_contract")
        .add_attribute("new_electors_contract", new_addr))
}

pub fn execute_register_elector(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {

    let msg = to_json_binary(&json!({
        "register": {
            "elector_address": info.sender.to_string()
        }
    }))?;
    
    let exec = WasmMsg::Execute {
        contract_addr: ELECTORS_CONTRACT.load(deps.storage)?.to_string(),
        msg,
        funds: vec![],
    };
    
    Ok(Response::new()
        .add_message(exec)
        .add_attribute("action", "register_elector")
        .add_attribute("elector_address", info.sender))
}

pub fn execute_follow_politician(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    politician_address: String,
) -> Result<Response, ContractError> {

    let politician = load_politician(deps.as_ref(), &info.sender)?; 

    let msg = to_json_binary(&json!({
        "follow_politician": {
            "elector_address": info.sender.to_string(),
            "role": politician.role,
            "politician_address": politician_address
        }
    }))?;
    
    let exec = WasmMsg::Execute {
        contract_addr: ELECTORS_CONTRACT.load(deps.storage)?.to_string(),
        msg,
        funds: vec![],
    };
    
    Ok(Response::new()
        .add_message(exec)
        .add_attribute("action", "elector_follow")
        .add_attribute("role", politician.role.to_string())
        .add_attribute("politician_address", politician_address))
}