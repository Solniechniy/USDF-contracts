use crate::{Contract, ContractExt};
use near_sdk::{env::log_str, is_promise_success, near};

#[near]
impl Contract {
    #[private]
    pub fn on_exchange_transfer(&mut self) {
        if !is_promise_success() {
            log_str("ERR_TRANSFER_FAILED");
            // TODO: Graceful error handling
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
