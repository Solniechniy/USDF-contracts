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
near deploy $CONTRACT_ID res/contract.wasm
```

### Initialize contract

- public_key: public key of the owner
- token_id: token id (USDF)
- withdraw_delay: delay in seconds for withdraw

```bash
near call $CONTRACT_ID new '{"public_key": [], "token_id": "'$TOKEN_ID'", "withdraw_delay": 600}' --accountId $OWNER_ID
```

### Add whitelisted token

```bash
near call $CONTRACT_ID add_whitelisted_token '{"token_id":
"'$TOKEN2_ID'"}' --accountId $OWNER_ID
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