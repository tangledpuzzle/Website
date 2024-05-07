use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("Already Registered")]
    AlreadyRegistered {},

    #[error("Registrations Closed")]
    RegistrationsClosed {},

    #[error("Wrong Payment")]
    WrongPayment {},

    #[error("Must pay by Juno")]
    MustPayByJuno {},

    #[error("Not sufficient funds")]
    NotSufficientFunds {},

    #[error("Raffle expired")]
    RaffleExpired {},

    #[error("Raffle not ended")]
    RaffleNotEnded {},

    #[error("Must pay by cw20 tokens")]
    MustPayByToken{},

    #[error("Expire setting is wrong")]
    WrongExpire {}
}
