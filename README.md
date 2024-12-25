# Solana Wallet Smart Contract

This project implements a basic wallet smart contract on the Solana blockchain using the Anchor framework.

## Features

- Initialize a new wallet
- Deposit funds into the wallet
- Withdraw funds from the wallet
- Balance tracking

## Smart Contract Structure

The smart contract consists of three main instructions:

1. `initialize`: Creates a new wallet account
2. `deposit`: Adds funds to the wallet
3. `withdraw`: Removes funds from the wallet (only callable by the wallet owner)

## Testing

The `tests` directory contains a set of tests that cover the basic functionality of the wallet smart contract. Run these tests using the `anchor test` command.
