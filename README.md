# MMOSH Smart Contract

## Software requirement
1. ubuntu 22.04 or mac os
2. Rust stable 1.75 +
3. Solana 1.18.9
4. Anchor 0.30.0
5. nodejs stable 18+

## Account Configuration Steps
1. Open Anchor.toml from root directory
2. Change wallet path in anchor.toml file 

## Configure MMOSH token
1. download MMOSH program from github - https://github.com/mmosh-pit/MMOSH-program
2. create new token and copy the token address
3. update the token address in tests/web3Consts.ts
   <pre>  oposToken: new web3.PublicKey("TOKEN_ADDRESS")</pre>

## Amman configuration Steps
1 Configure amman as root user
   <pre>npm install -g @metaplex-foundation/amman</pre>
3. Download metaplex js from github- https://github.com/metaplex-foundation/js
4. Run following command to run local solana validator along with metaplex contracts
    <pre>amman start .ammanrc.js</pre>

## Test Case Command
1. Install npm
   <pre>npm install </pre>
1. To run test with deployment 
   <pre>anchor test --skip-local-validator </pre>
2. To run test without deployment
   <pre>anchor test --skip-local-validator --skip-deploy </pre>

## Smart contract functions

| No | Function | Description | Documentation |
| :--- | :--- | :--- | :--- |
| `1` | `init_main_state` | `Create new PDA account to manage profile nft` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/_main/instructions/init_main_state/index.html) |
| `2` | `update_main_state` | `Update cost and share of profile PDA account` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/_main/instructions/update_main_state/index.html |
| `3` | `update_main_state_owner` | `Tranfser PDA account ownership` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/_main/instructions/update_main_state_owner/index.html |
| `4` | `create_collection` | `Create new collection for profile and pass` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/collection_factory/instructions/create_collection/index.html |
| `5` | `mint_gensis_profile` | `Create new gensis profile nft` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/profile/instructions/mint_genesis_profile/index.html |
| `6` | `mint_profile_by_at` | `Create new profile nft` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/profile/instructions/mint_profile_by_at/index.html |
| `7` | `update_profile` | `update profile nft metadata, name and symbol` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/profile/instructions/update_profile/index.html |
| `8` | `mint_gensis_pass` | `mint gensis pass along with new PDA account` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/profile/instructions/mint_gensis_pass/index.html |
| `9` | `mint_pass_by_at` | `mint pass nft` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/profile/instructions/mint_pass_by_at/index.html |
| `10` | `mint_guest_pass` | `mint new guest pass nft` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/profile/instructions/mint_guest_pass/index.html |
| `11` | `init_activation_token` | `Create new activation token account` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/activation_token/instructions/init_activation_token/index.html  |
| `12` | `mint_activation_token` | `Mint new activation token from profile nft` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/activation_token/instructions/mint_activation_token/index.html |
| `13` | `init_pass_token` | `Create new pass activation token account` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/activation_token/instructions/init_pass_token/index.html |
| `14` | `create_pass_token` | `Mint new pass activation token nft` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/activation_token/instructions/create_pass_token/index.html |
| `15` | `initialize_sol_storage_v0` | `Create new storage account for bonding curve` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/curve/instructions/initialize_sol_storage_v0/index.html |
| `16` | `create_curve_v0` | `Create new curve account` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/curve/instructions/create_curve_v0/index.html |
| `17` | `initialize_token_bonding_v0` | `Intialize token bonding account` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/curve/instructions/initialize_token_bonding_v0/index.html |
| `18` | `sell_v1` | `Burn memecoin to get token back from bonding curve` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/curve/instructions/sell/sell_v1/index.html |
| `19` | `buy_v1` | `Mint memecoin to lock token into bonding curve` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/curve/instructions/buy/buy_v1/index.html |
| `20` | `init_vault` | `Intialize new vault account` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/vault/instructions/init_vault/index.html |
| `21` | `stake_vault` | `stake value into vault account` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/vault/instructions/stake_vault/index.html |
| `22` | `unstake_vault` | `unstake value from vault account` | [link](https://htmlpreview.github.io/?https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/vault/instructions/unstake_vault/index.html |


## LICENSE
Copyright 2024 mmosh.app

Licensed under the GNU License, Version 3.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

https://www.gnu.org/licenses/agpl-3.0.en.html

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
