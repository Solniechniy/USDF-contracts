use crate::{errors::ERR_MUST_BE_OWNER, Contract, ContractExt};
use near_sdk::{env, near, require, AccountId};

#[near]
impl Contract {
    pub fn owner(&self) -> &AccountId {
        &self.owner
    }

    pub fn transfer_ownership(&mut self, owner: AccountId) {
        self.assert_owner();
        self.owner = owner;
    }
}

impl Contract {
    pub(crate) fn assert_owner(&self) {
        require!(
            env::predecessor_account_id().eq(self.owner()),
            ERR_MUST_BE_OWNER
        );
    }
}
