use std::cmp;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Raffle, COUNTER, RAFFLEMAP, ADMINS, Counter, JoinInfo};
use cosmwasm_std::{StdResult, Deps, Binary, QueryRequest, BankQuery, to_binary, AllBalanceResponse, from_binary, WasmMsg, CosmosMsg};

use rand_core::{RngCore, SeedableRng};
use crate::rand::{sha_256, Prng};
use rand_chacha::ChaChaRng;
use cw20::{Cw20ReceiveMsg, Cw20ExecuteMsg};

#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, Addr, BankMsg, Coin, DepsMut, Env, MessageInfo, Response, Uint128,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let admins: StdResult<Vec<_>> = msg
        .admins
        .into_iter()
        .map(|addr| deps.api.addr_validate(&addr))
        .collect();
    ADMINS.save(deps.storage, &admins?)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::BeginRaffleRound {
            expire_type, 
            minimum_stake, 
            winners_distribution,
            pay_token
        } => begin_raffle_round(deps, env, info, expire_type, minimum_stake, winners_distribution, pay_token),
        ExecuteMsg::JoinRaffleRoundByJuno {
            id
        } => join_raffle_round_by_juno(deps, env, info, id),
        ExecuteMsg::JoinRaffleRoundByToken (msg) => join_raffle_round_by_token(deps, env, info, msg),
        ExecuteMsg::EndRaffleRound {id,} => choose_winners(deps, env, info, id),
    }
}

pub fn begin_raffle_round(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    expire_type: u8,
    minimum_stake: Uint128,
    winners_distribution: Vec<u32>,
    pay_token: Option<String>
) -> Result<Response, ContractError>{
    if !is_admin(deps.as_ref(), info.sender.clone())? {
        return Err(ContractError::Unauthorized {});
    }

    let counter = COUNTER.load(deps.storage);
    
    let id;
    if let Ok(counter) = counter {
        id = counter.counter + 1;
    } else {
        id = 0;
    }
    
    COUNTER.save(deps.storage, &Counter { counter: id})?;

    let end_time_stamp = match expire_type {
        0 => env.block.time.plus_seconds(1800),
        1 => env.block.time.plus_seconds(3600),
        2 => env.block.time.plus_seconds(24*3600),
        3 => env.block.time.plus_seconds(24*3600*7),
        _ => return Err(ContractError::WrongExpire {})
    };

    let raffle = Raffle {
        id,
        begin_time_stamp: env.block.time,
        end_time_stamp,
        minimum_stake,
        winners_distribution,
        winners: Vec::new(),
        players: Vec::new(),
        winner_payouts: Vec::new(),
        active: true,
        pay_token,
    };

    RAFFLEMAP.save(deps.storage, &id.to_string(), &raffle)?;

    Ok(Response::default())
}



pub fn join_raffle_round_by_juno(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: u32,
) -> Result<Response, ContractError> {
    let mut raffle = RAFFLEMAP.load(deps.storage, &id.to_string())?;

    if raffle.is_expired(&env.block) {
        return Err(ContractError::RaffleExpired {});
    }

    if raffle.pay_token.is_some() {
        return Err(ContractError::MustPayByToken {});
    }
    
    if info.funds.len() != 1 {
        return Err(ContractError::WrongPayment {});
    }

    if info.funds[0].denom != "ujuno" {
        return Err(ContractError::MustPayByJuno {});
    }

    if info.funds[0].amount < raffle.minimum_stake {
        return Err(ContractError::NotSufficientFunds {});
    }

    if !can_register(deps.as_ref(), id)? {
        return Err(ContractError::RegistrationsClosed {});
    }

    if is_registered(deps.as_ref(), id, info.sender.to_string())? {
        return Err(ContractError::AlreadyRegistered {});
    }

    raffle.players.push(info.sender.to_string());
    RAFFLEMAP.save(deps.storage, &id.to_string(), &raffle)?;

    Ok(Response::default())
}

pub fn join_raffle_round_by_token(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    wrapper: Cw20ReceiveMsg,
) -> Result<Response, ContractError>  {
    let msg: JoinInfo = from_binary(&wrapper.msg)?;
    let id = msg.id;

    let mut raffle = RAFFLEMAP.load(deps.storage, &id.to_string())?;

    if raffle.pay_token.is_none() {
        return Err(ContractError::WrongPayment {  } );
    }
    let token_addr = raffle.clone().pay_token.unwrap();

    let token_addr = deps.api.addr_validate(token_addr.as_str())?;
    if token_addr != info.sender {
        return Err(ContractError::WrongPayment {  } );
    }
       
    if raffle.is_expired(&env.block) {
        return Err(ContractError::RaffleExpired {});
    }

    if wrapper.amount < raffle.minimum_stake {
        return Err(ContractError::NotSufficientFunds {});
    }

    if !can_register(deps.as_ref(), id)? {
        return Err(ContractError::RegistrationsClosed {});
    }

    if is_registered(deps.as_ref(), id, wrapper.sender.to_string())? {
        return Err(ContractError::AlreadyRegistered {});
    }

    raffle.players.push(wrapper.sender.to_string());
    RAFFLEMAP.save(deps.storage, &msg.id.to_string(), &raffle)?;

    Ok(Response::default())
}

pub fn choose_winners(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: u32,
) -> Result<Response, ContractError> {
    if !is_admin(deps.as_ref(), info.clone().sender)? {
        return Err(ContractError::Unauthorized {});
    }

    let raffle = RAFFLEMAP.load(deps.storage, &id.to_string())?;

    if !raffle.is_expired(&env.block) {
        return Err(ContractError::RaffleNotEnded {});
    }

    let prng_seed: Vec<u8> = sha_256(base64::encode("entropy").as_bytes()).to_vec();
    let random_seed = new_entropy(&info, &env, prng_seed.as_ref(), prng_seed.as_ref());
    let mut rng = ChaChaRng::from_seed(random_seed);

    let nb_players = raffle.players.len() as u32;
    let total_shares = raffle.clone().winners_distribution.iter().sum::<u32>();

    let total_deposit = query_total_deposit(deps.as_ref(), env)?;

    let res = Response::new();
    let mut winner_addresses = vec![];
    let mut payouts = vec![];

    for counter in 0..cmp::min(raffle.winners_distribution.len(), nb_players as usize) {
        let id_winner = (rng.next_u32() % nb_players) as usize;

        let winner_address = raffle.players[id_winner].to_owned();

        winner_addresses.push(winner_address.clone());

        let reward_per_share = total_deposit.checked_div(Uint128::from(total_shares)).unwrap();
        println!("total_shares: {}, reward_per_share: {}", total_shares, reward_per_share);

        let reward = reward_per_share.checked_mul(Uint128::from(raffle.winners_distribution[counter])).unwrap();
        println!("reward: {}", reward);
      
        payouts.push(reward);

        if raffle.clone().pay_token.is_none() {
            res.clone().add_message(BankMsg::Send { 
                to_address: winner_address, 
                amount: vec![Coin {
                    denom: String::from("ujuno"),
                    amount: reward,
                }]
            });
        } else {
            res.clone().add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: raffle.clone().pay_token.unwrap(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer { recipient: winner_address, amount: reward }).unwrap(),
                funds: vec![]
            }));
        }

    }

    let data = Raffle {
        id,
        begin_time_stamp: raffle.begin_time_stamp,
        end_time_stamp: raffle.end_time_stamp,
        players: raffle.players,
        winners: winner_addresses,
        minimum_stake: raffle.minimum_stake,
        winners_distribution: raffle.winners_distribution,
        winner_payouts: payouts,
        active: false,
        pay_token: raffle.pay_token,
    };

    RAFFLEMAP.save(deps.storage, &id.to_string(), &data)?;

    Ok(res)
}

pub fn is_admin(
    deps: Deps,
    addr: Addr,
) -> Result<bool, ContractError> {
    let admins = ADMINS.load(deps.storage)?;
    let is_admin = admins.contains(&addr);
    Ok(is_admin)
}

fn can_register(deps: Deps, id_lottery: u32) -> Result<bool, ContractError> {
    let raffle = RAFFLEMAP.load(deps.storage, &id_lottery.to_string())?;
    return Ok(raffle.active);
}

fn is_registered(deps: Deps, id_lottery: u32, caller: String) -> Result<bool, ContractError> {
    let raffle = RAFFLEMAP.may_load(deps.storage, &id_lottery.to_string())?;

    if raffle.unwrap().players.contains(&caller) {
        return Ok(true);
    } else {
        return Ok(false);
    }
}

pub fn new_entropy(info: &MessageInfo, env: &Env, seed: &[u8], entropy: &[u8]) -> [u8; 32] {
    // 16 here represents the lengths in bytes of the block height and time.
    let entropy_len = 16 + info.sender.to_string().len() + entropy.len();
    let mut rng_entropy = Vec::with_capacity(entropy_len);
    rng_entropy.extend_from_slice(&env.block.height.to_be_bytes());
    rng_entropy.extend_from_slice(&info.sender.as_bytes());
    rng_entropy.extend_from_slice(entropy);

    let mut rng = Prng::new(seed, &rng_entropy);

    rng.rand_bytes()
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTotalDeposit { } => to_binary(&query_total_deposit(deps, env)?),
        QueryMsg::GetCount {  } => to_binary(&get_current_counter(deps)?),
        QueryMsg::GetRaffleInfo { id } => to_binary(&get_raffle_info(deps, id)?)
    }
}

pub fn query_total_deposit(deps: Deps, env: Env) -> StdResult<Uint128>{
    let balance: AllBalanceResponse  = deps.querier.query(
        &QueryRequest::Bank(BankQuery::AllBalances{
            address: env.contract.address.to_string(),
        })
    )?;

    let juno_amount = balance.amount
     .iter()
     .find(|c|c.denom =="ujuno".to_string())
     .map(|c|c.amount)
     .unwrap_or_else(Uint128::zero);
    println!("juno amount: {}", juno_amount);
    Ok(juno_amount)
}

fn get_current_counter(deps: Deps) -> StdResult<u32> {
    let counter = COUNTER.load(deps.storage)?;
    Ok(counter.counter)
}

fn get_raffle_info(deps:Deps, id: u32) -> StdResult<Raffle> {
    let raffle = RAFFLEMAP.load(deps.storage, &id.to_string())?;
    Ok(raffle)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary, Uint128, Coin};
    use crate::ContractError;
    use crate::contract::{instantiate, execute, query};
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
    use crate::state::Raffle;
    
    #[test]
    fn begin_raffle_round() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &[]);

        let instantiate_msg = InstantiateMsg {
            admins: vec!["creator".to_string()]
        };

        instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

        let raffle_msg = ExecuteMsg::BeginRaffleRound {
            expire_type: 0,
            minimum_stake: Uint128::from(10 as u32),
            winners_distribution: vec![1, 2, 3],
            pay_token: None
        };
        
        let env = mock_env();
        execute(deps.as_mut(), env.clone(), info, raffle_msg).unwrap();

        let query_msg = QueryMsg::GetRaffleInfo { id: 0};
        let res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let raffle: Raffle = from_binary(&res).unwrap();

        assert_eq!(raffle, Raffle {
            id: 0,
            begin_time_stamp: env.block.time,
            minimum_stake:  Uint128::from(10 as u32),
            end_time_stamp: env.block.time.plus_seconds(1800),
            winners_distribution: vec![1, 2, 3],
            players: vec![],
            winner_payouts: vec![],
            winners: vec![],
            active: true, 
            pay_token: None, 
        });
    } 

    #[test]
    fn join_raffle_round() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &[]);

        let instantiate_msg = InstantiateMsg {
            admins: vec!["creator".to_string()]
        };

        instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

        // begin a raffle
        let raffle_msg = ExecuteMsg::BeginRaffleRound {
            expire_type: 0,
            pay_token: None, 
            minimum_stake: Uint128::from(10 as u32),
            winners_distribution: vec![1, 2, 3],
        };
        
        let env = mock_env();
        execute(deps.as_mut(), env.clone(), info, raffle_msg).unwrap();

        // join the raffle
        let join_raffle_msg = ExecuteMsg::JoinRaffleRoundByJuno {
            id: 0
        };

        let info = mock_info("player", &[]);
        let err = execute(deps.as_mut(), mock_env(), info, join_raffle_msg.clone()).unwrap_err();
        
        match err {
            ContractError::WrongPayment { } => { },
            e => panic!("unexpected error: {}", e),
        }

        let info = mock_info("player", &[Coin{ denom: "ujuno".to_string(), amount: Uint128::from(10 as u128)}]);
        execute(deps.as_mut(), mock_env(), info, join_raffle_msg.clone()).unwrap();
        
        let query_msg = QueryMsg::GetRaffleInfo { id: 0};
        let res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let raffle: Raffle = from_binary(&res).unwrap();

        assert_eq!(raffle, Raffle {
            id: 0,
            begin_time_stamp: env.block.time,
            minimum_stake:  Uint128::from(10 as u32),
            end_time_stamp: env.block.time.plus_seconds(1800),
            winners_distribution: vec![1, 2, 3],
            players: vec!["player".to_string()],
            winner_payouts: vec![],
            winners: vec![],
            active: true,
            pay_token: None
        });

        let info = mock_info("player", &[Coin{ denom: "ujuno".to_string(), amount: Uint128::from(10 as u128)}]);
        let err = execute(deps.as_mut(), mock_env(), info, join_raffle_msg.clone()).unwrap_err();
        match err {
            ContractError::AlreadyRegistered { } => { },
            e => panic!("unexpected error: {}", e),
        }

        let info = mock_info("player1", &[Coin{ denom: "ujuno".to_string(), amount: Uint128::from(1 as u128)}]);
        let err = execute(deps.as_mut(), mock_env(), info, join_raffle_msg).unwrap_err();
        match err {
            ContractError::NotSufficientFunds { } => { },
            e => panic!("unexpected error: {}", e),
        }   
    } 

    #[test]
    fn choose_winners() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &[]);

        let instantiate_msg = InstantiateMsg {
            admins: vec!["creator".to_string()]
        };

        instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

        // begin a raffle
        let raffle_msg = ExecuteMsg::BeginRaffleRound {
            expire_type: 0,
            pay_token: None, 
            minimum_stake: Uint128::from(10 as u32),
            winners_distribution: vec![5, 3, 2]
        };
        
        let env = mock_env();
        execute(deps.as_mut(), env.clone(), info, raffle_msg).unwrap();

        // join the raffle
        let join_raffle_msg = ExecuteMsg::JoinRaffleRoundByJuno  {
            id: 0
        };
        let info = mock_info("player", &[Coin{ denom: "ujuno".to_string(), amount: Uint128::from(1_000_000 as u128)}]);
        execute(deps.as_mut(), mock_env(), info, join_raffle_msg.clone()).unwrap();
     
        // join the raffle #2
        let join_raffle_msg = ExecuteMsg::JoinRaffleRoundByJuno {
            id: 0
        };
        let info = mock_info("player2", &[Coin{ denom: "ujuno".to_string(), amount: Uint128::from(2_000_000 as u128)}]);
        execute(deps.as_mut(), mock_env(), info, join_raffle_msg.clone()).unwrap();

        // end the raffle
        let end_raffle_msg = ExecuteMsg::EndRaffleRound {
            id: 0
        };
        let info = mock_info("creator", &[]);
        execute(deps.as_mut(), mock_env(), info, end_raffle_msg.clone()).unwrap();
        
        // let query_msg = QueryMsg::GetRaffleInfo { id: 0};
        // let res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        // let raffle: Raffle = from_binary(&res).unwrap();

        // assert_eq!(raffle, Raffle {
        //     id: 0,
        //     begin_time_stamp: env.block.time,
        //     minimum_stake:  Uint128::from(500_000 as u32),
        //     end_time_stamp: Timestamp::from_nanos(2_000_000_000_000_000_000),
        //     winners_distribution: vec![5, 3, 2],
        //     players: vec!["player".to_string()],
        //     winner_payouts: vec![Uint128::from(10 as u32)],
        //     winners: vec!["player".to_string()],
        //     active: false 
        // });
    } 
}

