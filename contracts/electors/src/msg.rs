use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String, // 👈 Vamos usar String aqui, convertida depois para Addr
}

/// Mensagens para executar ações que alteram o estado
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    /// Cadastra um novo eleitor (se ainda não estiver registrado)
    Register {},

    /// Dá follow em um político, em uma das funções (vereador, deputado, etc)
    FollowPolitician {
        role: PoliticalRole,
        politician_address: Addr,
    },

    // Adiciona saldo (futuramente só pelo sistema de recompensa)
    AddBalance {
        amount: u128,
    },
}

/// Possíveis cargos políticos que um eleitor pode seguir
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum PoliticalRole {
    Vereador,
    DeputadoEstadual,
    Governador,
    DeputadoFederal,
    Senador,
    Presidente,
}

/// Mensagens para consultas (não alteram o estado)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    /// Consulta os dados de um eleitor
    GetElector { address: Addr },

    /// Consulta apenas o saldo de um eleitor
    GetBalance { address: Addr },
}
