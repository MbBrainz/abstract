use crate::contract::instantiate;
use crate::tests::common::{execute_as, TEST_CREATOR};
use crate::tests::mock_querier::mock_dependencies;
use abstract_core::ans_host::*;
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{Addr, DepsMut, MessageInfo};
use speculoos::prelude::*;

pub(crate) fn instantiate_msg(info: &MessageInfo) -> InstantiateMsg {
    InstantiateMsg {
        admin: info.sender.to_string(),
    }
}

/**
 * Mocks instantiation.
 */
pub fn mock_instantiate(deps: DepsMut) {
    let info = mock_info(TEST_CREATOR, &[]);
    let msg = InstantiateMsg {
        admin: info.sender.to_string(),
    };

    let _res = instantiate(deps, mock_env(), info, msg)
        .expect("contract successfully handles InstantiateMsg");
}

/**
 * Tests successful instantiation of the contract.
 */
#[test]
fn successful_initialization() {
    let mut deps = mock_dependencies(&[]);

    let info = mock_info(TEST_CREATOR, &[]);
    let msg = instantiate_msg(&info);
    let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
}

#[test]
fn successful_update_ownership() {
    let mut deps = mock_dependencies(&[]);
    mock_instantiate(deps.as_mut());

    let new_admin = "new_admin";
    // First update to transfer
    let transfer_msg = ExecuteMsg::UpdateOwnership(cw_ownable::Action::TransferOwnership {
        new_owner: new_admin.to_string(),
        expiry: None,
    });

    let transfer_res = execute_as(deps.as_mut(), TEST_CREATOR, transfer_msg).unwrap();
    assert_eq!(0, transfer_res.messages.len());

    // Then update and accept as the new owner
    let accept_msg = ExecuteMsg::UpdateOwnership(cw_ownable::Action::AcceptOwnership);
    let accept_res = execute_as(deps.as_mut(), new_admin, accept_msg).unwrap();
    assert_eq!(0, accept_res.messages.len());

    assert_that!(cw_ownable::get_ownership(&deps.storage).unwrap().owner)
        .is_some()
        .is_equal_to(Addr::unchecked(new_admin))
}
