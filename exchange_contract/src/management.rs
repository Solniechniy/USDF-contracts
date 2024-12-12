use near_sdk::{near, AccountId};

use crate::{Contract, ContractExt, TimeExtension};

#[near]
impl Contract {
    pub fn update_withdraw_delay(&mut self, withdraw_delay: u64) {
        self.assert_owner();
        self.withdraw_delay = withdraw_delay.to_nanoseconds();
    }

    pub fn add_whitelisted_token(&mut self, token_id: AccountId) {
        self.assert_owner();
        self.whitelisted_tokens.push(token_id);
    }

    pub fn remove_whitelisted_token(&mut self, token_id: AccountId) {
        self.assert_owner();
        let index = self
            .whitelisted_tokens
            .iter()
            .position(|t| t == &token_id)
            .expect("ERR_NO_TOKEN");
        self.whitelisted_tokens.remove(index);
    }

    pub fn delete_user(&mut self, account_id: AccountId) {
        self.assert_owner();
        self.users.remove(&account_id);
    }
}
