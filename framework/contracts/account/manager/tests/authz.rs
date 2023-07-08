mod common;

use abstract_adapter::mock::{MockExecMsg, MockInitMsg};
use abstract_core::manager::ManagerModuleInfo;
use abstract_core::objects::fee::FixedFee;
use abstract_core::objects::module::{ModuleInfo, ModuleVersion, Monetization};
use abstract_core::{adapter::BaseQueryMsgFns, *};
use abstract_interface::*;
use abstract_testing::prelude::{OWNER, TEST_ACCOUNT_ID, TEST_MODULE_ID, TEST_VERSION};
use common::*;
use cosmwasm_std::{coin, coins};
use cosmwasm_std::{Addr, Coin, Empty};
use cw_orch::deploy::Deploy;
use cw_orch::prelude::*;
use cw_orch::test_tube::OsmosisTestTube;
// use cw_multi_test::StakingInfo;
use speculoos::{assert_that, result::ResultAssertions, string::StrAssertions};

use crate::common::mock_modules::{BootMockAdapter1V1, BootMockAdapter1V2, V1, V2};

#[test]
fn installing_one_adapter_should_succeed() -> AResult {
    let sender = Addr::unchecked(common::OWNER);
    let chain = OsmosisTestTube::new(coins(100000000000, "ueur"));
    let deployment = Abstract::deploy_on(chain.clone(), Empty {})?;
    let account = create_default_account(&deployment.account_factory)?;
    let staking_adapter = init_mock_adapter(chain, &deployment, None)?;
    install_adapter(&account.manager, TEST_MODULE_ID)?;

    let modules = account.expect_modules(vec![staking_adapter.address()?.to_string()])?;

    assert_that(&modules[1]).is_equal_to(&ManagerModuleInfo {
        address: staking_adapter.address()?,
        id: TEST_MODULE_ID.to_string(),
        version: cw2::ContractVersion {
            contract: TEST_MODULE_ID.into(),
            version: TEST_VERSION.into(),
        },
    });

    // Configuration is correct
    let adapter_config = staking_adapter.config()?;
    assert_that!(adapter_config).is_equal_to(adapter::AdapterConfigResponse {
        ans_host_address: deployment.ans_host.address()?,
        dependencies: vec![],
        version_control_address: deployment.version_control.address()?,
    });

    // no authorized addresses registered
    let authorized = staking_adapter.authorized_addresses(account.proxy.addr_str()?)?;
    assert_that!(authorized)
        .is_equal_to(adapter::AuthorizedAddressesResponse { addresses: vec![] });

    Ok(())
}
