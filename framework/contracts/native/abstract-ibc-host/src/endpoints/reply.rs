use crate::{
    account_commands::{self},
    state::{REGISTRATION_CACHE, RESULTS},
    HostError,
};
use abstract_core::objects::AccountId;
use abstract_sdk::core::abstract_ica::{DispatchResponse, RegisterResponse, StdAck};
use cosmwasm_std::{DepsMut, Env, Reply, Response};

pub const RECEIVE_DISPATCH_ID: u64 = 1234;
pub const INIT_CALLBACK_ID: u64 = 7890;

pub fn reply_dispatch_callback(
    deps: DepsMut,
    _env: Env,
    reply: Reply,
) -> Result<Response, HostError> {
    // add the new result to the current tracker
    let mut results = RESULTS.load(deps.storage)?;
    results.push(reply.result.unwrap());
    RESULTS.save(deps.storage, &results)?;

    // update result data if this is the last
    let data = StdAck::success(DispatchResponse { results });
    Ok(Response::new().set_data(data))
}

/// Handle reply after the Account is created, reply with the proxy address of the created account.
pub fn reply_init_callback(deps: DepsMut, _env: Env, _reply: Reply) -> Result<Response, HostError> {
    // we use storage to pass info from the caller to the reply
    let (_, account_id): (String, AccountId) = REGISTRATION_CACHE.load(deps.storage)?;
    REGISTRATION_CACHE.remove(deps.storage);
    // get the account for the callback
    let account = account_commands::get_account(deps.as_ref(), &account_id)?;

    let data = StdAck::success(RegisterResponse {
        /// return the proxy address of the created account, this allows for coin transfers
        account: account.proxy.into_string(),
    });
    Ok(Response::new().set_data(data))
}
