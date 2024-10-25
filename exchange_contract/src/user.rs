use near_contract_standards::fungible_token::{core::ext_ft_core, Balance};
use near_sdk::{env, json_types::U128, near, require, AccountId, NearToken, Promise};

use crate::{on_transfer::ExchangeData, withdraws::Withdraw, Contract};
pub const ONE_YOCTO: Balance = 1;

#[near(serializers = [borsh, serde])]
pub struct User {
    pub nonce: u64,
    pub exchanges: Vec<ExchangeItem>,
}

#[near(serializers = [borsh, serde])]
pub struct ExchangeItem {
    pub token_in: AccountId,
    pub amount_in: u128,
    pub amount_out: u128,
}

impl Contract {
    pub fn execute_exchange(&mut self, data: ExchangeData) -> Promise {
        self.assert_whitelisted_token(&data.token_in);

        if !self.users.contains_key(&data.account_id) {
            self.users.insert(
                data.account_id.clone(),
                User {
                    nonce: 0,
                    exchanges: vec![],
                },
            );
        }

        let user = self.users.get_mut(&data.account_id).expect("ERR_NO_USER");

        user.nonce = data.nonce;

        user.exchanges.push(ExchangeItem {
            token_in: data.token_in.clone(),
            amount_in: data.amount_in,
            amount_out: data.amount_out,
        });

        ext_ft_core::ext(self.token_id.clone())
            .with_attached_deposit(NearToken::from_yoctonear(ONE_YOCTO))
            .ft_transfer(data.account_id, U128(data.amount_out), None)
            .then(Self::ext(env::current_account_id()).on_exchange_transfer())
    }

    pub fn reverse_exchange(&mut self, amount: U128, account_id: AccountId, token_id: AccountId) {
        let user = self.users.get_mut(&account_id).expect("ERR_USER_NOT_FOUND");

        // find id of exchange item with amount_out == amount
        let exchange_id = user
            .exchanges
            .iter()
            .position(|e| e.amount_out == amount.0)
            .expect("ERR_EXCHANGE_ITEM_NOT_FOUND");

        // remove exchange item from user
        let exchange_item = user.exchanges.remove(exchange_id);

        require!(exchange_item.token_in == token_id, "ERR_INVALID_TOKEN_IN");

        self.withdraw_list.push(Withdraw {
            account_id: account_id.clone(),
            amount: exchange_item.amount_in,
            token_id: exchange_item.token_in,
            timestamp: env::block_timestamp(),
        });
    }
}

impl Contract {
    pub(crate) fn assert_whitelisted_token(&self, token: &AccountId) {
        assert!(
            self.whitelisted_tokens.contains(token),
            "ERR_NOT_WHITELISTED_TOKEN"
        );
    }

    pub(crate) fn validate_data(
        &self,
        data: &ExchangeData,
        account_id: &AccountId,
        token_id: &AccountId,
        amount: u128,
    ) {
        assert!(
            data.amount_in > 0 && data.amount_out > 0,
            "ERR_INVALID_AMOUNTS"
        );

        let nonce = self.users.get(account_id).map(|u| u.nonce).unwrap_or(0);
        assert!(data.nonce > nonce, "ERR_INVALID_NONCE");

        assert!(data.token_in == *token_id, "ERR_INVALID_TOKEN_IN");

        assert!(amount == data.amount_in, "ERR_INVALID_AMOUNT");
    }
}
