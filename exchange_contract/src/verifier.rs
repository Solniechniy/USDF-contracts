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
        stream.append(&data.amount_out.0.to_be_bytes().as_ref());
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

#[cfg(test)]
use ed25519_dalek::{ed25519::signature::SignerMut, SigningKey};

#[test]
fn sign_msg() {
    let mut stream = RlpStream::new_list(5);
    let mut hasher = Sha256::new();

    let nonce: u128 = 9;
    let token_in: AccountId = "poken.sergei24.testnet".parse().unwrap();
    let amount: u128 = 2000000000000000000;
    let usdf_amount: u128 = 4200000000000000000;
    let user_address: AccountId = "boca.testnet".parse().unwrap();

    stream.append(&nonce.to_be_bytes().as_ref());
    stream.append(&token_in.as_bytes());
    stream.append(&amount.to_be_bytes().as_ref());
    stream.append(&usdf_amount.to_be_bytes().as_ref());
    stream.append(&user_address.as_bytes());

    let message = stream.out().to_vec();

    println!("stream.out().to_vec() : {:?}", message);
    hasher.update(message);

    let msg = hasher.finalize().to_vec();

    println!("hasher.finalize().to_vec() : {:?}", msg);

    let secret_key =
        "qYWT9DhyXarXSKtjUmdNZfU299itX63uF15werFZVfYw1atK8kKTxnZKL7BiqbjRmZ6Y7zQsKRGasm5Kgk3Rx64"
            .to_string();

    let vec = near_sdk::bs58::decode(&secret_key).into_vec().unwrap();
    let slice: [u8; 64] = vec.as_slice().try_into().unwrap();

    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&slice[..32]);

    println!("first 32 bytes of PK  {:?}", bytes);

    let mut signing_key = SigningKey::from_bytes(&bytes);
    let signature: Signature = signing_key.sign(&msg);

    println!("{:?}", signature.to_bytes().to_vec());

    // check signature
    let public_key = "2bAAQpg3bUcNMU6t2vtst9wFhU3pD9HDQeDHNMYENfA8";

    let public_key_bytes = near_sdk::bs58::decode(&public_key)
        .into_vec()
        .expect(ERR_VERIFYING_KEY);
    let public_key_slice: &[u8; 32] = public_key_bytes
        .as_slice()
        .try_into()
        .expect(ERR_VERIFYING_KEY);
    let verifying_key = VerifyingKey::from_bytes(public_key_slice).expect(ERR_VERIFYING_KEY);
    let signature =
        Signature::from_bytes(&signature.clone().try_into().expect(ERR_PARSE_SIGNATURE));

    require!(
        verifying_key.verify(&msg, &signature).is_ok(),
        ERR_INVALID_SIGNATURE
    );
}
