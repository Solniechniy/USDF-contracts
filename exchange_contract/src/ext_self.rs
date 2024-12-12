use crate::{Contract, ContractExt};
use near_sdk::{env::log_str, is_promise_success, near, AccountId};

#[near]
impl Contract {
    #[private]
    pub fn on_exchange_transfer(&mut self, account_id: AccountId) {
        if !is_promise_success() {
            log_str("ERR_TRANSFER_FAILED");
            let user = self.users.get_mut(&account_id).expect("ERR_NO_USER");

            user.exchanges.pop();

            if user.exchanges.is_empty() {
                self.users.remove(&account_id);
            }
        }
        // TODO: Add event
    }

    #[private]
    pub fn on_reverse_exchange_transfer(&mut self) {
        if !is_promise_success() {
            log_str("ERR_TRANSFER_FAILED");
            // TODO: Graceful error handling
        }
        // TODO: Add event
    }
}
