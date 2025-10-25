## EVM
This Project is made with foundry and contains the contracts relevant to the protocol.

**Foundry is a blazing fast, portable and modular toolkit for Ethereum application development written in Rust.**

Foundry consists of:

-   **Forge**: Ethereum testing framework (like Truffle, Hardhat and DappTools).
-   **Cast**: Swiss army knife for interacting with EVM smart contracts, sending transactions and getting chain data.
-   **Anvil**: Local Ethereum node, akin to Ganache, Hardhat Network.
-   **Chisel**: Fast, utilitarian, and verbose solidity REPL.

## Documentation

https://book.getfoundry.sh/

## Usage

Update the submodules first 
```shell
git submodule update --init --recursive
```

### Build

```shell
forge build
```

### Test

```shell
forge test
```

### Format

```shell
forge fmt
```

### Gas Snapshots

```shell
forge snapshot
```

### Anvil

```shell
anvil
```

### Deploy

```shell
source .
forge clean
forge script script/DeployLocker.s.sol:DeployLocker \
    --rpc-url $RPC_URL \
    --broadcast \
    --private-key $PRIVATE_KEY \
    --force
```


### Upgrade

```shell
source .
forge clean
forge script script/UpgradeLocker.s.sol:DeployLocker \
    --rpc-url $RPC_URL \
    --broadcast \
    --private-key $PRIVATE_KEY \
    --force
```

### Cast

```shell
cast <subcommand>
```

### Help

```shell
forge --help
anvil --help
cast --help
```
