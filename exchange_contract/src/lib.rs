use ed25519_dalek::PUBLIC_KEY_LENGTH;
use errors::ERR_VERIFYING_KEY;
use near_sdk::{env, near, store::LookupMap, AccountId, BorshStorageKey, PanicOnDefault};
use on_transfer::TimeExtension;
use user::User;
use withdraws::Withdraw;

mod errors;
mod events;
mod ext_self;
mod management;
mod on_transfer;
mod owner;
mod user;
mod verifier;
mod withdraws;

#[derive(BorshStorageKey)]
#[near(serializers = [borsh])]
pub enum StorageKey {
    Users,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    pub owner: AccountId,
    pub pub_signer_key: [u8; PUBLIC_KEY_LENGTH],
    pub token_id: AccountId,
    pub whitelisted_tokens: Vec<AccountId>,
    pub users: LookupMap<AccountId, User>,
    pub withdraw_list: Vec<Withdraw>,
    pub withdraw_delay: u64,
}

#[near]
impl Contract {
    #[init]
    pub fn new(public_key: Vec<u8>, token_id: AccountId, withdraw_delay: u64) -> Self {
        Self {
            owner: env::predecessor_account_id(),
            pub_signer_key: public_key.try_into().expect(ERR_VERIFYING_KEY),
            token_id,
            whitelisted_tokens: vec![],
            users: LookupMap::new(StorageKey::Users),
            withdraw_list: vec![],
            withdraw_delay: withdraw_delay.to_nanoseconds(),
        }
    }
}
