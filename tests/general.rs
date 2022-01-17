use near_sdk::json_types::U128;
use near_sdk_sim::{call, deploy, init_simulator, ContractAccount, to_yocto, UserAccount};
use test_get_promise_result::TestGetPromiseResultContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    CONTRACT_BYTES => "target/wasm32-unknown-unknown/release/test_get_promise_result.wasm",
}

fn init() -> (UserAccount, ContractAccount<TestGetPromiseResultContract>) {
  let mut genesis = near_sdk_sim::runtime::GenesisConfig::default();
  genesis.gas_limit = u64::MAX;
  genesis.gas_price = 0;
  let master_account = init_simulator(Some(genesis));
  let contract_account = deploy! {
        contract: TestGetPromiseResultContract,
        contract_id: "contract",
        bytes: &CONTRACT_BYTES,
        signer_account: master_account
    };
  (master_account, contract_account)
}

#[test]
fn test_transfer_with_reference() {
  let (master_account, contract) = init();
  let result = call!(
    master_account,
    contract.transfer_with_reference(
      U128::from(to_yocto("1"))
    )
  );
  println!("{:#?}", result.logs());
}
