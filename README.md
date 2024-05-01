# soma_token_vault

This is a Solana on chain program that implement a token vault.

What it could do:
- Create a new SPL token which is governed by the on chain program.
- Can Mint token to user (Airdrop).
- Can let users stake their token in the stake pool.
- Can let users unstake and withdraw token after locking period.

## Illustration

![Token Vault Program](https://raw.githubusercontent.com/airicyu/soma_token_vault/main/img/token_vault.png)

----

## Technical Implementation

Anchor version: 0.29.0

## From install setup to run local test

(It is an Anchor program so you can do in anchor way if you know)

### Install/Setup

1. Make sure you have installed Rust.
2. Make sure you have installed Solana-CLI.
3. Make sure you have installed Anchor-CLI. (0.29.0)
4. Assume your (1)-(3) env setup is OK.
5. Checkout this program.
6. "npm i" to install.
7. Update the program address for the anchor program. (You can google for how to do this.)

### How to run local test
1. Run "npm run test".

(You can also run your local validator separately.)