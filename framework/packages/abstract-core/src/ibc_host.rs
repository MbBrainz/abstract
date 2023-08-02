//! # Abstract Api Base
//!
//! `abstract_core::adapter` implements shared functionality that's useful for creating new Abstract adapters.
//!
//! ## Description
//! An Abstract adapter contract is a contract that is allowed to perform actions on a [proxy](crate::proxy) contract.
//! It is not migratable and its functionality is shared between users, meaning that all users call the same contract address to perform operations on the Account.
//! The api structure is well-suited for implementing standard interfaces to external services like dexes, lending platforms, etc.

use crate::{
    ibc_client::CallbackInfo,
    manager,
    objects::{account::AccountId, chain_name::ChainName},
};
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::{Addr, Binary, CosmosMsg, Empty, QueryRequest};

/// Used by Abstract to instantiate the contract
/// The contract is then registered on the version control contract using [`crate::version_control::ExecuteMsg::ProposeModules`].
#[cosmwasm_schema::cw_serde]
pub struct InstantiateMsg {
    /// Used to easily perform address translation on the app chain
    pub ans_host_address: String,
    /// Used to create remote abstract accounts
    pub account_factory_address: String,
    /// Version control address
    pub version_control_address: String,
}

#[cosmwasm_schema::cw_serde]
pub struct MigrateMsg {}

#[cosmwasm_schema::cw_serde]
pub enum InternalAction {
    /// Registers a new account on the remote chain
    Register {
        account_proxy_address: String,
        name: String,
        description: Option<String>,
        link: Option<String>,
    },
    /// Message used to know the name of the counterparty chain with which a channel was just created
    WhoAmI {
        // Client chain to assign to the channel
        client_chain: ChainName,
    },
}

/// Callable actions on a remote host
#[cosmwasm_schema::cw_serde]
pub enum HostAction {
    App {
        msg: Binary,
    },
    Dispatch {
        manager_msg: manager::ExecuteMsg,
    },
    Query {
        msgs: Vec<QueryRequest<Empty>>,
    },
    SendAllBack {},
    Balances {},
    /// Can't be called through the packet endpoint directly
    Internal(InternalAction),
}

impl HostAction {
    pub fn into_packet(
        self,
        account_id: AccountId,
        retries: u8,
        host_chain: ChainName,
        callback_info: Option<CallbackInfo>,
    ) -> PacketMsg {
        PacketMsg {
            host_chain,
            retries,
            callback_info,
            account_id,
            action: self,
        }
    }
}
/// This is the message we send over the IBC channel
#[cosmwasm_schema::cw_serde]
pub struct PacketMsg {
    /// `ChainName` of the host
    pub host_chain: ChainName,
    /// Amount of retries to attempt if packet returns with StdAck::Error
    pub retries: u8,
    pub account_id: AccountId,
    /// Callback performed after receiving an StdAck::Result
    pub callback_info: Option<CallbackInfo>,
    /// execute the custom host function
    pub action: HostAction,
}

/// Interface to the Host.
#[cosmwasm_schema::cw_serde]
pub enum ExecuteMsg {
    /// Update the Admin
    UpdateAdmin { admin: String },
    UpdateConfig {
        ans_host_address: Option<String>,
        account_factory_address: Option<String>,
        version_control_address: Option<String>,
    },
    /// Allow for fund recovery through the Admin
    RecoverAccount {
        closed_channel: String,
        account_id: AccountId,
        msgs: Vec<CosmosMsg>,
    },
}

/// Query Host message
#[cosmwasm_schema::cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns [`ConfigResponse`].
    #[returns(ConfigResponse)]
    Config {},
}

#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {
    pub ans_host_address: Addr,
    pub account_factory_address: Addr,
    pub version_control_address: Addr,
}

#[cfg(test)]
mod tests {
    use super::*;
    use abstract_testing::prelude::*;
    use speculoos::prelude::*;

    #[test]
    fn test_into_packet() {
        // Create an instance of HostAction (assuming a valid variant is `SomeAction`)
        let host_action = HostAction::SendAllBack {};

        // Create required parameters
        let retries = 5u8;
        let client_chain = String::from("test_client_chain");
        let callback_info = Some(CallbackInfo {
            id: "15".to_string(),
            receiver: "receiver".to_string(),
        });

        // Call into_packet function
        let packet_msg = host_action.clone().into_packet(
            TEST_ACCOUNT_ID,
            retries,
            client_chain.clone(),
            callback_info.clone(),
        );

        // Check if the returned PacketMsg has the expected values
        assert_that!(packet_msg.client_chain).is_equal_to(client_chain);
        assert_that!(packet_msg.retries).is_equal_to(retries);
        assert_that!(packet_msg.callback_info).is_equal_to(callback_info);
        assert_that!(packet_msg.account_id).is_equal_to(TEST_ACCOUNT_ID);
        assert_that!(packet_msg.action).is_equal_to(host_action);
    }
}
