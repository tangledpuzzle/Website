#[cfg(test)]
mod tests {
  use super::*;
  use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MOCK_CONTRACT_ADDR};
  use cosmwasm_std::{attr, coins, CosmosMsg};

#[test]
fn create_raffle_object()
{
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("creator", &[]);
    let id = 1;
    let endTimeStamp = Timestamp::from_seconds(1000000000);
    let players = Vec::new();
    let minimumStake = Uint128::from(1000000u128);
    let winnersDistribution = vec![1, 2, 3];
    let staking_native = true;
    let res = handle::begin_raffle_round(deps.as_mut(), env, info, id, endTimeStamp, players, minimumStake, winnersDistribution, staking_native);
    assert_eq!(0, res.unwrap().messages.len());
}}
