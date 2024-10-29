use near_contract_standards::fungible_token::{core::ext_ft_core, Balance};
use near_sdk::{env, json_types::U128, near, require, AccountId, NearToken, PromiseOrValue};

use crate::{on_transfer::ExchangeData, withdraws::Withdraw, Contract};
pub const ONE_YOCTO: Balance = 1;

pub type Nonce = u64;

#[derive(Clone)]
#[near(serializers = [borsh, json])]
pub struct User {
    pub nonce: Nonce,
    pub exchanges: Vec<ExchangeItem>,
}

#[derive(Clone)]
#[near(serializers = [borsh, json])]
pub struct ExchangeItem {
    pub token_in: AccountId,
    pub amount_in: Balance,
    pub amount_out: Balance,
}

impl Contract {
    pub fn execute_exchange(
        &mut self,
        data: ExchangeData,
        token_in: &AccountId,
        amount_in: u128,
        account_id: &AccountId,
    ) -> PromiseOrValue<U128> {
        self.assert_whitelisted_token(token_in);

        if !self.users.contains_key(account_id) {
            self.users.insert(
                account_id.clone(),
                User {
                    nonce: 0,
                    exchanges: vec![],
                },
            );
        }

        let user = self.users.get_mut(account_id).expect("ERR_NO_USER");

        user.nonce = data.nonce;

        user.exchanges.push(ExchangeItem {
            token_in: token_in.clone(),
            amount_in,
            amount_out: data.amount_out,
        });

        ext_ft_core::ext(self.token_id.clone())
            .with_attached_deposit(NearToken::from_yoctonear(ONE_YOCTO))
            .ft_transfer(account_id.clone(), U128(data.amount_out), None)
            .then(Self::ext(env::current_account_id()).on_exchange_transfer(account_id.clone()));

        PromiseOrValue::Value(U128(0))
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

    pub(crate) fn validate_data(&self, data: &ExchangeData, account_id: &AccountId) {
        assert!(data.amount_out > 0, "ERR_INVALID_AMOUNT_OUT");

        let nonce = self.users.get(account_id).map(|u| u.nonce).unwrap_or(0);
        assert!(data.nonce > nonce, "ERR_INVALID_NONCE");
    }
}
