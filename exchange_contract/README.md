# Exchange contract

## Build

```bash
./build-docker.sh
```

## Test

```bash
cargo test
```

## Accounts

```bash
export NEAR_ENV=mainnet
export OWNER_ID=owner.near
export CONTRACT_ID=contract.near
export TOKEN1_ID=token.near
export TOKEN2_ID=token.near
export USER_ID=user.near
```

### For creating the new account for deploying contract use next command

```bash
near create-account $CONTRACT_ID --masterAccount $OWNER_ID --initialBalance 5
```

### Deploy contract

```bash
near deploy $CONTRACT_ID res/exchange_contract.wasm
```

### Initialize contract

- public_key: public key of the owner
- token_id: token id (USDF)
- withdraw_delay: delay in seconds for withdraw

```bash
near call $CONTRACT_ID new '{"public_key": [23,155,203,82,140,189,208,103,153,249,146,121,110,27,45,241,39,161,219,185,176,240,163,43,187,195,205,210,244,86,144,97], "token_id": "'$TOKEN2_ID'", "withdraw_delay": 600}' --accountId $OWNER_ID
```

### Add whitelisted token

```bash
near call $CONTRACT_ID add_whitelisted_token '{"token_id": "usmeme.tg"}' --accountId $OWNER_ID
```

### Update withdraw delay

```bash
near call $CONTRACT_ID update_withdraw_delay '{"withdraw_delay": 600}' --accountId $OWNER_ID
```

### Remove whitelisted token

```bash
near call $CONTRACT_ID remove_whitelisted_token '{"token
_id": "'$TOKEN2_ID'"}' --accountId $OWNER_ID
```

### Exchange tokens

To exchange tokens you need to call ft_transfer_call with structure

```rust
pub struct ExchangeData {
    pub amount_out: u128,
    pub nonce: u64,
    pub signature: Vec<u8>,
}
```

```bash
near call $TOKEN2_ID ft_transfer_call '{"receiver_id": "'$CONTRACT_ID'", "amount": "100000000000000000000", "msg": "{\"amount_out\": \"10000000000000000000\", \"nonce\": 1, \"signature\": []}"}' --accountId $OWNER_ID --gas 280000000000000 --depositYocto 1
```

### Exchange back

To exchange back usdf tokens to asset you have to send exactly the same count of tokens as you received. Msg no matter.

```bash
near call $TOKEN1_ID ft_transfer_call '{"receiver_id": "'$CONTRACT_ID'", "amount": "100000000000000000000", "msg": ""}' --accountId $OWNER_ID --gas 280000000000000 --depositYocto 1
```

### Get withdraws count

```bash
near view $CONTRACT_ID get_withdraws_count '{}' --accountId $OWNER_ID
```

### Get withdraws

```bash
near view $CONTRACT_ID get_withdraws '{"from": 0, "limit": 10}' --accountId $OWNER_ID
```

### Get whitelisted tokens

```bash
near view $CONTRACT_ID get_whitelisted_tokens '{}' --accountId $OWNER_ID
```

### Get withdraw delay

```bash
near view $CONTRACT_ID get_withdraw_delay '{}' --accountId $OWNER_ID
```

### Get user

```bash
near view $CONTRACT_ID get_user '{"account_id": "'$USER_ID'"}' --accountId $OWNER_ID
```

### Get withdraws by user

```bash
near view $CONTRACT_ID get_withdraws_by_user '{"account_id": "'$USER_ID'"}' --accountId $OWNER_ID
```

### delete user

```bash
near call $CONTRACT_ID delete_user '{"account_id": "'$USER_ID'"}' --accountId $OWNER_ID --gas 300000000000000
```
