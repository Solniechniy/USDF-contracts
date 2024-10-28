use near_contract_standards::fungible_token::core::ext_ft_core;
use near_sdk::{env, json_types::U128, near, require, AccountId, NearToken, Promise};

use crate::{user::ONE_YOCTO, Contract, ContractExt};

#[derive(Clone)]
#[near(serializers = [borsh, json])]
pub struct Withdraw {
    pub account_id: AccountId,
    pub amount: u128,
    pub token_id: AccountId,
    pub timestamp: u64,
}

#[near]
impl Contract {
    pub fn withdraw(&mut self) -> Promise {
        let account_id = env::predecessor_account_id();
        let withdraw = self
            .withdraw_list
            .iter()
            .position(|w| w.account_id == account_id)
            .expect("ERR_NO_WITHDRAW");

        let withdraw = self.withdraw_list.remove(withdraw);

        require!(
            withdraw.timestamp + self.withdraw_delay < env::block_timestamp(),
            "ERR_WITHDRAW_DELAY"
        );

        ext_ft_core::ext(self.token_id.clone())
            .with_attached_deposit(NearToken::from_yoctonear(ONE_YOCTO))
            .ft_transfer(account_id, U128(withdraw.amount), None)
            .then(Self::ext(env::current_account_id()).on_reverse_exchange_transfer())
    }
}
