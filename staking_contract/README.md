# USMeme SC

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
export STAKING_ID=staking.usdf.sergei24.testnet
export TOKEN1_ID=token.near
export NFT_ID=nft.near
export USER_ID=user.near
```

### For creating the new account for deploying contract use next command

```bash
near create-account $STAKING_ID --masterAccount $OWNER_ID --initialBalance 5
```

### Deploy contract

```bash
near deploy $STAKING_ID ../res/staking_contract.wasm
```

### Initialize contract



```bash
near call $STAKING_ID new '{ "config": { "usdf_contract_id": "'$TOKEN1_ID'", "yusdf_contract": "'$TOKEN3_ID'", "apy": 1000}}' --accountId $OWNER_ID
```

## Contract management


### Update FT contract id

- ft_contract_id - account id of the FT contract

```bash
near call $STAKING_ID update_yusdf_contract_id '{"yusdf_contract": "'$TOKEN_ID'"}' --accountId $OWNER_ID
```

### Add tokens

- to add tokens to the contract, you need to call the `ft_transfer_call` method of the FT contract with action `AddDeposit`

```bash
near call $TOKEN3_ID storage_deposit '{}' --accountId $STAKING_ID --amount 0.0125

```bash
near call $TOKEN3_ID ft_transfer_call '{"receiver_id": "'$STAKING_ID'", "amount": "110000000000000000000", "msg": "{\"AddDeposit\":null}"}' --accountId $OWNER_ID --depositYocto 1 --gas 300000000000000
```

## View functions


### get_yusdf_balance

```bash
near view $STAKING_ID get_yusdf_balance
```

### Get user

- account_id - account id of the owner

-returns `Option<UserOutput>)`

```bash
near view $STAKING_ID get_user '{"account_id": "'$OWNER_ID'"}'
```

## User methods

### To stake tokens, you need to call the `ft_transfer_call` method of the FT contract with action `Stake`

```bash
near call $TOKEN1_ID ft_transfer_call '{"receiver_id": "'$STAKING_ID'", "amount": "10000000000000000000", "msg":"{\"Stake\":null}"}' --accountId $OWNER_ID --depositYocto 1 --gas 300000000000000
```

### To unstake tokens, you need to call the `ft_transfer_call` method of the FT contract with action `Unstake`

Note: You must transfer exactly the same amount of tokens that now is staked. You can unstake only all tokens at once.

```bash
near call $TOKEN1_ID ft_transfer_call '{"receiver_id": "'$STAKING_ID'", "amount": "10000000000000000000", "msg":"{\"Unstake\":null}"}' --accountId $OWNER_ID --depositYocto 1 --gas 300000000000000
```
