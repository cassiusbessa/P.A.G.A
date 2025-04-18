use crate::utils::only_owner;
use crate::state:: ELECTORS_CONTRACT;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, WasmMsg, to_json_binary};
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

    let electors_contract = ELECTORS_CONTRACT.load(deps.storage)?;

    let msg = electors::msg::ExecuteMsg::Register {
        elector_address: info.sender.clone(),
    };

    let exec = WasmMsg::Execute {
        contract_addr: electors_contract.to_string(),
        msg: to_json_binary(&msg)?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(exec)
        .add_attribute("action", "register_elector")
        .add_attribute("caller", info.sender.clone()))
}
