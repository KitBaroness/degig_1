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
| :--- | :--- | :--- |
| `1` | `init_main_state` | `Create new PDA account to manage profile nft` | [link](https://github.com/mmosh-pit/mmosh-smart-contract/blob/dev/doc/mmoshforge/_main/instructions/init_main_state/index.html) |
| `2` | `update_main_state` | `Update cost and share of profile PDA account` | `test` |
| `3` | `update_main_state_owner` | `Tranfser PDA account ownership` | `test` |
| `4` | `set_common_lut` | `Set common LUT function which used to reduce public key size while minting new profile nft` | `test` |
| `5` | `create_collection` | `Create new collection for profile and pass` | `test` |
| `6` | `mint_gensis_profile` | `Create new gensis profile nft` | `test` |
| `7` | `mint_profile_by_at` | `Create new profile nft` | `test` |
| `8` | `update_profile` | `update profile nft metadata, name and symbol` | `test` |
| `9` | `mint_gensis_pass` | `mint gensis pass along with new PDA account` | `test` |
| `10` | `mint_pass_by_at` | `mint pass nft` | `test` |
| `11` | `mint_guest_pass` | `mint new guest pass nft` | `test` |
| `12` | `init_activation_token` | `Create new activation token account` | `test` |
| `13` | `mint_activation_token` | `Mint new activation token from profile nft` | `test` |
| `14` | `init_pass_token` | `Create new pass activation token account` | `test` |
| `15` | `create_pass_token` | `Mint new pass activation token nft` | `test` |
| `16` | `initialize_sol_storage_v0` | `Create new storage account for bonding curve` | `test` |
| `17` | `create_curve_v0` | `Create new curve account` | `test` |
| `18` | `initialize_token_bonding_v0` | `Intialize token bonding account` | `test` |
| `19` | `sell_v1` | `Burn memecoin to get token back from bonding curve` | `test` |
| `20` | `buy_v1` | `Mint memecoin to lock token into bonding curve` | `test` |
| `21` | `init_vault` | `Intialize new vault account` | `test` |
| `22` | `stake_vault` | `stake value into vault account` | `test` |
| `23` | `unstake_vault` | `unstake value from vault account` | `test` |


## LICENSE
Copyright 2024 mmosh.app

Licensed under the GNU License, Version 3.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

https://www.gnu.org/licenses/agpl-3.0.en.html

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
