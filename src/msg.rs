use cosmwasm_std::{Addr, Uint128};
use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    BeginRaffleRound { 
        expire_type: u8,  // 0: 30 mins, 1: 1 hour, 2: 1 day, 3: 1 week
        minimum_stake: Uint128,
        winners_distribution: Vec<u32>,
        pay_token: Option<String>,
    },
    JoinRaffleRoundByJuno {
        id: u32
    },
    JoinRaffleRoundByToken (
        Cw20ReceiveMsg
    ),
    EndRaffleRound {id: u32},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
    // GetWinner {},
    GetTotalDeposit {},
    GetRaffleInfo { id: u32 },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetCountResponse {
    pub count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetWinnerResponse {
    pub winner: Vec<Addr>,
}
