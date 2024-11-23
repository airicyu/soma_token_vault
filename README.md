# soma_token_vault

Precaution: This is just for learning purpose. Dun use it for production!! Take your own risk!!

This is a Solana on chain program that implement a token vault.

What it could do:
- Create a new SPL token which is governed by the on chain program.
- Can Mint token to user (Airdrop).
- Can let users stake their token in the stake pool.
- Can let users unstake and withdraw token after locking period.

## Illustration

![Token Vault Program](https://raw.githubusercontent.com/airicyu/soma_token_vault/main/img/token_vault.png)

----

## From install setup to run local test

(It is an Anchor program so you can do in anchor way if you know)

### Install/Setup

1. Make sure you have installed Rust.
2. Make sure you have installed Solana-CLI. (Update Anchor.toml's toolchain solana_version to your local solana version, recommend 1.18.x)
3. Make sure you have installed Anchor-CLI. (0.29.0)
4. Checkout this program.
5. "npm i" to install.
6. Update the program address for the anchor program. (You can google for how to do this.)

### How to run local test
1. Run "npm run test".

(You can also run your local validator separately.)

----


## Staking

The program allow users to stake their token into the pool to earn. The earn rewards are come from the reward vault.

User can stake at any time and can stake more after staked. When user want to take back the staked token, they need to do "unstake" first and wait for a certain locking period. After the locking period, user can perform a "withdraw" instruction to withdraw the unstaked tokens.

### Stake pool

The centralize pool which user stake token into.

### Vault (Reward vault)

This is a vault that storing reward tokens for the staking.

----

## Program Instructions Explain

### initalizeProgram

The first program init instruction to run after program deployed.

It initialize below accounts:

- owner_config
- relayer_config
- minter
  - The PDA that would set as the token authority
- vault
  - The vault account. This account is the owner and also storing some metadata like balances.
- stake_pool
  - The stake pool account. This account is the owner and also storing some metadata like balances.

instruction data:

```rust
pub struct InitializeProgramInstructionData {
    pub owner: Pubkey,
    pub relayer: Pubkey,
}
```
Just passing owner address and relayer address.

----

###  initializeToken

The second program init instruction.

It initialize below accounts:

- mint
  - Create the token being used for the whole program protocol
- vault_token_account
  - The vault's token account. Owned by vault account
- stake_pool_token_account
  - The stake pool's token account. Owned by stake_pool account.

instruction data:

- token_decimals
  - The token's decimals. the whole program are all process with atomic unit of token. Given the token decimal is 4, then an amount of 10000 atomic unit of token actually means 1 token in UI.

----

### mintToVaule

Minting token to the vault. Only can call by relayer.

instruction data:
- amount
  - amount of token to mint

----

### burnFromVault

Burning token from the vault. Only can call by relayer.

instruction data:
- amount
  - amount of token to burn.

----

### mintToUserAta

Minting token to the user. Only can call by relayer.

The instruction would need to pass the target user and user token account via instruction accounts.

```rust
#[account()]
pub user: AccountInfo<'info>,

#[account(mut, 
    associated_token::mint = mint,
    associated_token::authority = user,
    associated_token::token_program = token_program
)]
pub user_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
```

instruction data:
- amount
  - amount of token to mint.

----

### stake

This instruction is for users to call to stake their own tokens.

It would transfer token from user's ATA to the stake pool's token account. And we would create a PDA per user to store user stake info.

instruction data:
- amount
  - amount of token to stake.

----

### unstake

instruction data:
- amount
  - amount of token to unstake.

----

### withdraw

The instruction would withdraw all unstaked token.

It would transfer the principal tokens from stake pool back to user. And the extra rewards earned would also be minted to user from vault.

----

### changeStakeRate

The instruction would update the stake rate config.

instruction data:
- stake_base_rate
  - the new stake rate

----

### changeOwner

The instruction change the owner (Be careful!)

instruction data:
- onwer
  - the new owner 

----

### changeRelayer

The instruction change the relayer (Be careful!)

instruction data:
- relayer
  - the new relayer

------

## Questions/Notes

### Q: Why many account is wrapped by "Box"?

If not wrapped by "Box", it may encounter some error related to exceed stake size in rust. Wrapping with Box would make the objects store on the heap, which has a much larger size limit.

