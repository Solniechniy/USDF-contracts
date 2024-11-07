use {
    near_contract_standards::fungible_token::{receiver::FungibleTokenReceiver, Balance},
    near_sdk::{env, json_types::U128, near, serde_json, AccountId, PromiseOrValue},
};

use crate::{user::Nonce, Contract, ContractExt};

#[near(serializers = [json])]
pub struct ExchangeData {
    pub amount_out: U128,
    pub nonce: Nonce,
    pub signature: Vec<u8>,
}

#[near]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let token_in = env::predecessor_account_id();
        if token_in == self.token_id {
            self.reverse_exchange(amount, sender_id, token_in);

            PromiseOrValue::Value(U128(0))
        } else {
            let data: ExchangeData = serde_json::from_str(&msg).expect("ERR_FAILED_TO_PARSE_MSG");

            self.verify_signature(&data, &token_in, amount.0, &sender_id);

            self.validate_data(&data, &sender_id);

            self.execute_exchange(data, &token_in, amount.0, &sender_id)
        }
    }
}

pub trait TimeExtension {
    fn to_nanoseconds(&self) -> u64;
}

impl TimeExtension for u64 {
    fn to_nanoseconds(&self) -> u64 {
        let ns_multiplier = 1_000_000_000;
        self * ns_multiplier
    }
}
