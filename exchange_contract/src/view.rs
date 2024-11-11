use near_sdk::{near, AccountId};

use crate::{user::User, withdraws::Withdraw, Contract, ContractExt};

#[near]
impl Contract {
    pub fn get_withdraws_count(&self) -> usize {
        self.withdraw_list.len()
    }

    pub fn get_withdraws(&self, from: u16, limit: u16) -> Vec<Withdraw> {
        let to_index = std::cmp::min(from + limit, self.withdraw_list.len() as _);

        self.withdraw_list[from as usize..to_index as usize].to_vec()
    }

    pub fn get_whitelisted_tokens(&self) -> Vec<AccountId> {
        self.whitelisted_tokens.clone()
    }

    pub fn get_withdraw_delay(&self) -> u64 {
        self.withdraw_delay
    }

    pub fn get_user(&self, account_id: AccountId) -> Option<User> {
        self.users.get(&account_id).cloned()
    }

    pub fn get_withdraws_by_user(&self, account_id: AccountId) -> Vec<Withdraw> {
        self.withdraw_list
            .iter()
            .filter(|w| w.account_id == account_id)
            .cloned()
            .collect()
    }
}
