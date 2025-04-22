use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Addr};
use crate::state::{Elector, ELECTORS};
use crate::errors::ContractError;
use crate::msg::PoliticalRole;
use crate::utils::only_paga;
use alloc::string::ToString;



/// Função que registra um novo eleitor, se ainda não existir
pub fn execute_register(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    elector_address: Addr,
) -> Result<Response, ContractError> {
    only_paga(deps.as_ref(), &info)?;

    // Verifica se o eleitor já existe
    if ELECTORS.has(deps.storage, &elector_address) {
        return Err(ContractError::AlreadyRegistered {});
    }

    let new_elector = Elector {
        address: elector_address.clone(),
        balance: 0,
        follows: Default::default(), // todas as posições vazias
    };

    ELECTORS.save(deps.storage, &elector_address, &new_elector)?;

    Ok(Response::new()
        .add_attribute("action", "register")
        .add_attribute("elector", elector_address))
}

/// Função que faz o eleitor seguir um político (com base no cargo)
pub fn execute_follow(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    elector_address: Addr,
    role: PoliticalRole,
    politician_address: Addr,
) -> Result<Response, ContractError> {

    only_paga(deps.as_ref(), &info)?;

    let sender = info.sender;

    // Verifica se o eleitor está registrado
    let mut elector = ELECTORS
        .load(deps.storage, &elector_address)
        .map_err(|_| ContractError::NotRegistered {})?;

    // Clona o endereço do político para usar várias vezes
    let politician = politician_address.clone();

    // Atualiza o político seguido com base no cargo
    match role {
        PoliticalRole::Vereador => elector.follows.vereador = Some(politician),
        PoliticalRole::DeputadoEstadual => elector.follows.deputado_estadual = Some(politician),
        PoliticalRole::Governador => elector.follows.governador = Some(politician),
        PoliticalRole::DeputadoFederal => elector.follows.deputado_federal = Some(politician),
        PoliticalRole::Senador => elector.follows.senador = Some(politician),
        PoliticalRole::Presidente => elector.follows.presidente = Some(politician),
    }

    // Salva de volta o eleitor atualizado
    ELECTORS.save(deps.storage, &elector_address, &elector)?;

    Ok(Response::new()
        .add_attribute("action", "follow")
        .add_attribute("follower", sender)
        .add_attribute("politician", politician_address)
        .add_attribute("role", role.to_string()))
}

