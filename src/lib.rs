use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::{env, ext_contract, Gas, log, near_bindgen, Promise, PromiseError};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Default)]
pub struct TestGetPromiseResult;

#[ext_contract(ext)]
pub trait ExtCrossContract {
    fn get_rate(p1: u8, p2: String) -> u32;
    fn rate_callback(#[callback_result] get_rate: Result<u32, PromiseError>, amount: U128) -> bool;
}

#[near_bindgen]
impl TestGetPromiseResult {
    pub fn transfer_with_reference(&mut self, amount: U128) -> Promise {
        // some logic

        ext::get_rate(
            1,
            "oracle_account_id".to_string(),
            env::current_account_id(),
            0,
            Gas(5_000_000_000_000)
        )
          //.and( ... ) Call other promises and some logic
          .then(
              ext::rate_callback(
                  amount,
                  env::current_account_id(),
                  0,
                  Gas(5_000_000_000_000)
              )
          )
    }

    #[private]
    pub fn get_rate(p1: u8, p2: String) -> u32 {
        log!("{}, {}", p1, p2);
        12
    }

    #[private]
    pub fn rate_callback(#[callback_result] get_rate: Result<u32, PromiseError>, amount: U128)
        -> U128
    {
        let rate = match get_rate {
            Ok(v) => v,
            _ => env::panic_str("Error getting rate")
        };
        let result = U128::from(amount.0 * rate as u128);
        env::log_str(
            &json!({
                "amount": result,
            }).to_string().as_str()
        );
        // some logic
        result
    }
}
