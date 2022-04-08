# Astral Charter Smart Contract
The smart contract back-end to support the Astral Charter education application

## Overview
Astral Charter, located at [astralcharter.com](https://astralcharter.com), is an online education platform that is designed to bring learning to the blockchain. In Astral Charter, users are able to take various missions (courses) and are awarded with badges upon successfully completing the course quiz. These badges are stored permanently on the NEAR blockchain, and will be mintable NFTs in a future release. 

The code in this repository is for the underlying smart contracts that power Astral Charter. 

## Structure
This contract is written in Rust programming language, and was developed using the Visual Studio Code IDE. The address for the smart contract is `development-jeffreybrown.testnet`. 

## Installation
1. Install Rustup
Run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

2. Configure your current shell
Run `source $HOME/.cargo/env`

Note: alternatively you can simply relaunch your terminal window

3. Add wasm target to your toolchain
Run `rustup target add wasm32-unknown-unknown`

## Authentication
You'll need to authenticate to your NEAR wallet before proceeding. 
Run `near login` and follow the prompts to successfully authenticate

## Methods
Your account details object on the NEAR blockchain stores your completed missions and associated badge data. There are two methods to interact with this: `set_account_details` and `get_account_details`. 

### Set Account Details
The `set_account_details` method takes a string as an argument to `account`, which should be in the form of a JSON object. 
For example, to add `{"name": "FirstName}` to your account, you would run:
`near call development-jeffrey.testnet set_account_details '{"account": "{"name": "FirstName"}}' --accountId YOURWALLETADDRESSHERE`

### Get Account Details
The `get_account_details` method returns the entire value of the "account" object on the blockchain. 
To get the current value of the account object, run `near view development-jeffreybrown.testnet get_account_details '{"account_id": "YOURWALLETADDRESSHERE"}'`