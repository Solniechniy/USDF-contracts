use crate::{
    errors::{ERR_INVALID_SIGNATURE, ERR_PARSE_SIGNATURE, ERR_VERIFYING_KEY},
    on_transfer::ExchangeData,
    Contract,
};
use {
    ed25519_dalek::{Signature, Verifier, VerifyingKey},
    near_sdk::{require, AccountId},
    rlp::RlpStream,
    sha2::{Digest, Sha256},
};

impl Contract {
    pub fn verify_signature(
        &self,
        data: &ExchangeData,
        token_in: &AccountId,
        amount_in: u128,
        account_id: &AccountId,
    ) {
        let mut stream = RlpStream::new_list(5);
        let mut hasher = Sha256::new();

        stream.append(&data.nonce.to_be_bytes().as_ref());
        stream.append(&token_in.as_bytes());
        stream.append(&amount_in.to_be_bytes().as_ref());
        stream.append(&data.amount_out.to_be_bytes().as_ref());
        stream.append(&account_id.as_bytes());

        let message = stream.out().to_vec();
        hasher.update(message);

        let msg_to_verify = hasher.finalize().to_vec();

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
            verifying_key.verify(&msg_to_verify, &signature).is_ok(),
            ERR_INVALID_SIGNATURE
        );
    }
}
