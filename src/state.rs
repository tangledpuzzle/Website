use cosmwasm_std::BlockInfo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::Item;
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Raffle
{
    pub id : u32,
    pub begin_time_stamp : Timestamp,
    pub end_time_stamp : Timestamp,
    pub minimum_stake : Uint128, // Size per slot
    pub winners_distribution: Vec<u32>,
    pub players: Vec<String>,
    pub winner_payouts: Vec<Uint128>,
    pub winners : Vec<String>,
    pub active: bool,
    pub pay_token: Option<String>,
}

impl Raffle {
    pub fn is_expired(&self, block: &BlockInfo) -> bool {
        self.end_time_stamp <= block.time
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct JoinInfo {
    pub id: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Counter
{
    pub counter: u32,
}

pub const STATE: Item<Raffle> = Item::new("raffle");
pub const COUNTER: Item<Counter> = Item::new("counter");
pub const RAFFLEMAP: Map<&str, Raffle> = Map::new("escrow");
pub const ADMINS: Item<Vec<Addr>> = Item::new("admins");
pub const PLAYERS: Item<Vec<Addr>> = Item::new("players");