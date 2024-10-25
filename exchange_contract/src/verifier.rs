use crate::{
    errors::{ERR_INVALID_SIGNATURE, ERR_PARSE_SIGNATURE, ERR_VERIFYING_KEY},
    on_transfer::ExchangeData,
    Contract,
};
use {
    ed25519_dalek::{Signature, Verifier, VerifyingKey},
    near_sdk::require,
};

impl Contract {
    pub fn verify_signature(&self, data: &ExchangeData) {
        let msg_to_verify = format!(
            "{},{},{},{}",
            data.token_in, data.amount_in, data.amount_out, data.account_id
        );
        let verifying_key =
            VerifyingKey::from_bytes(&self.pub_signer_key).expect(ERR_VERIFYING_KEY);
        let signature = Signature::from_bytes(
            &data
                .signature
                .clone()
                .try_into()
                .expect(ERR_PARSE_SIGNATURE),
        );

        require!(
            verifying_key
                .verify(msg_to_verify.as_bytes(), &signature)
                .is_ok(),
            ERR_INVALID_SIGNATURE
        );
    }
}
