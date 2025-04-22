use cosmwasm_schema::cw_serde;
use alloc::string::String;
use crate::state::PoliticianRole;


#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String, // 👈 Vamos usar String aqui, convertida depois para Addr
    pub paga_contract: String, // 👈 Vamos usar String aqui, convertida depois para Addr
}

/// Mensagens para executar ações que alteram o estado
#[cw_serde]
pub enum ExecuteMsg {
    /// Registra um novo político com um papel específico
    RegisterPolitician {
        politician_address: String,
        role: PoliticianRole,
    },

    /// Cria uma nova promessa para o político chamador
    CreatePromise {
        politician_address: String,
        title: String,
        description: String,
        conclusion_date: Option<u64>,
    },

    /// Um eleitor vota a favor ou contra uma promessa
    VoteOnPromise {
        voter_address: String,
        politician_address: String,
        promise_id: u64,
        in_favor: bool,
    },
}


/// Mensagens para consultas (não alteram o estado)
#[cw_serde]
pub enum QueryMsg {
    /// Retorna as promessas de um político
    GetPromisesByPolitician {
        politician: String,
    },

    /// Retorna os detalhes de uma promessa específica
    GetPromise {
        politician: String,
        promise_id: u64,
    },

    /// Retorna os votos de um eleitor
    GetVotesByElector {
        elector: String,
    },

    /// Retorna os dados de um político
    GetPolitician {
        address: String,
    },
}

