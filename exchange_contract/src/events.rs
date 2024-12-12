use {
    near_sdk::{json_types::U128, near, serde::Serialize, AccountId},
    near_sdk_contract_tools::Nep297,
};

#[near(serializers = [json])]
pub enum ExchangeStatus {
    Success,
    Failed,
}

#[derive(Serialize, Nep297)]
#[serde(crate = "near_sdk::serde")]
#[nep297(standard = "exchange-event", version = "1.0.0")]
pub struct ExchangeEvent {
    pub account_id: AccountId,
    pub amount: U128,
    pub token_id: AccountId,
    pub status: ExchangeStatus,
}
