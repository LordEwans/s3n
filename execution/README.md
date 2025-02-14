# S3N AVS Node

For support contact @lordewan on telegram

## Introduction

The S3N node must be run with a Intel SGX with SGX2 enabled. If you want to use a cloud provider, please use one of these:

- Azure (https://learn.microsoft.com/en-us/azure/confidential-computing/quick-create-portal)
- OVH (https://help.ovhcloud.com/csm/en-dedicated-servers-intel-sgx?id=kb_article_view&sysparm_article=KB0044005)

We recommend you use Ubuntu 22.04

We DO NOT support AWS Enclaves!

## Check SGX

First we should check you are on a valid SGX machine.

Head over to https://support.fortanix.com/hc/en-us/articles/4414753648788-SGX-Detect-Tool and download the binary for your operating system.

Ubuntu 22.04:
`wget https://download.fortanix.com/sgx-detect/ubuntu22.04/sgx-detect?_gl=1*1j4w6dh*_gcl_au*MTM1MDY4MDQ2NS4xNzE4ODUwMTk1 -O sgx-detect`
`chmod +x sgx-detect`
`sudo ./sgx-detect`

You should see:
<Enter screenshot>

If you see any red on the output, please contact @EulerLagrange217 on telegram

## Register Node Operator with EigenLayer

The following is not S3N specific but for EigenLayer. We will be summarizing the following verboase guide: https://docs.eigenlayer.xyz/eigenlayer/operator-guides/operator-installation

1. Install EigenLayer-CLI

For Ubuntu 22.04:
`install-eigenlayer-cli`

2. Create ECDSA and BLS keys

`make generate-keys`

OR

`eigenlayer operator keys create --key-type ecdsa s3n`
`eigenlayer operator keys create --key-type bls s3n`

To check if you did it right you can run:

`make list-keys`

<insert image>

3. Fund ECDSA Wallet with testnet ETH

We need to load your operator wallet with some holesky eth to submit transactions. See: https://docs.eigenlayer.xyz/eigenlayer/restaking-guides/restaking-user-guide/testnet/obtaining-testnet-eth-and-liquid-staking-tokens-lsts

4. Use some Holesky eth to get holesky stETH:

https://stake-holesky.testnet.fi/

5. Operator config

We need to generate some metadata so eigenlayer can show you nicely in their UI.

I would recommend forking this repo: https://github.com/Hmac512/eigenpod and changing the values to what you want.

Run:

`eigenlayer operator config create`

Say yes to populating the config

> Enter your operator address: 0xfe8463ca0a9b436fdc5f75709ad5a43961802d68
> Enter your earnings address (default to your operator address): 0xfe8463ca0a9b436fdc5f75709ad5a43961802d68
> Enter your ETH rpc url: https://ethereum-holesky.publicnode.com
> Select your network: holesky
> Select your signer type: local_keystore
> Enter your ecdsa key path: /home/ubuntu/.eigenlayer/operator_keys/s3n.ecdsa.key.json

This will generate a operator.yaml file that should look like:

`operator:
    address: 0xFE8463CA0A9b436FdC5f75709AD5a43961802d68
    earnings_receiver_address: 0xFE8463CA0A9b436FdC5f75709AD5a43961802d68
    delegation_approver_address: "0x0000000000000000000000000000000000000000"
    staker_opt_out_window_blocks: 0
    metadata_url: "https://raw.githubusercontent.com/Hmac512/eigenpod/main/metadata.json?"
el_delegation_manager_address: 0xA44151489861Fe9e3055d95adC98FbD462B948e7
eth_rpc_url: https://ethereum-holesky.publicnode.com
private_key_store_path: /home/ubuntu/.eigenlayer/operator_keys/s3n.ecdsa.key.json
signer_type: local_keystore
chain_id: 17000
fireblocks:
    api_key: ""
    secret_key: ""
    base_url: ""
    vault_account_name: ""
    secret_storage_type: ""
    aws_region: ""
    timeout: 0
web3:
    url: ""`

Remember to fill out the metadata_url to your forked repo.

Make a backup of your operator.yaml for convenience

6. Register Node Operator

run:

`make register-eigen-operator`
OR
`eigenlayer operator register operator.yaml`
