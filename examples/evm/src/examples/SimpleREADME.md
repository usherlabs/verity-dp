# Simple ICP Signature Verification Example

This is a simplified, dependency-free implementation of ICP (Internet Computer Protocol) signature verification in Ethereum smart contracts, based on the original EVM Locker contract.

## Overview

The Simple ICP Signature Verifier demonstrates core ICP signature verification functionality without external dependencies, making it easy to understand, deploy, and integrate into existing projects.

## Key Features

- **Zero Dependencies**: No external libraries required
- **ICP Signature Verification**: Verify signatures from ICP canisters using ECDSA
- **Fund Management**: Deposit and withdraw native ETH with signature-based authorization
- **Replay Protection**: Prevent signature reuse with signature tracking
- **Simple Ownership**: Basic access control for contract administration
- **Comprehensive Testing**: Full test suite covering all functionality

## Contract Architecture

### Core Functions

```solidity
// Initialization
function initialize(address _icpCanister, string _chainId) external

// Fund Management
function depositTokens(string _canisterId) external payable
function withdrawTokens(string _canisterId, uint256 _nonce, uint256 _amount, bytes _signature) external
function cancelWithdraw(string _canisterId, uint256 _nonce, uint256 _amount, bytes _signature) external

// Signature Verification
function verifyICPSignature(bytes32 _dataHash, bytes _signature) public view returns (bool)
function verifyCustomMessage(string _message, bytes _signature) external view returns (bool)

// Administration
function setICPCanister(address _icpCanister) external
function transferOwnership(address _newOwner) external
function emergencyWithdraw(address _recipient) external

// Queries
function getBalance(string _canisterId) external view returns (uint256)
function getContractBalance() external view returns (uint256)
function isSignatureUsed(bytes _signature) external view returns (bool)
function getContractInfo() external view returns (address, address, string, bool)
```

## Usage Examples

### 1. Deployment

```bash
# Set environment variables
export ICP_CANISTER_ADDRESS="0x1234567890123456789012345678901234567890"
export CHAIN_ID="ethereum"

# Deploy using Foundry
forge script script/DeploySimpleICPSignatureVerifier.s.sol --rpc-url $RPC_URL --broadcast
```

### 2. Depositing Funds

```solidity
// Deposit native ETH for a canister
icpVerifier.depositTokens{value: 1000 ether}("rdmx6-jaaaa-aaaah-qcaiq-cai");
```

### 3. Withdrawing with ICP Signature

```solidity
// Create message hash (done by ICP canister)
bytes32 messageHash = keccak256(abi.encodePacked(
    nonce, amount, msg.sender, chainId, canisterId
));

// ICP canister signs and sends signature
bytes memory signature = icpSignature; // From ICP canister

// Withdraw funds
icpVerifier.withdrawTokens(canisterId, nonce, amount, signature);
```

### 4. Verifying Custom Messages

```solidity
// Verify arbitrary message signatures
bool isValid = icpVerifier.verifyCustomMessage("Hello World", signature);
```

## ICP Integration

### Setting Up ICP Canister

1. **Generate ECDSA Key**: Your ICP canister needs an ECDSA key for signing
2. **Derive Ethereum Address**: Convert the public key to an Ethereum address
3. **Implement Signing**: Create signature generation in your ICP canister
4. **Deploy Contract**: Use the derived address as the ICP canister address

### Signature Format

- **Length**: 65 bytes
- **Components**: r (32 bytes) + s (32 bytes) + v (1 byte)
- **Recovery ID**: v must be 27 or 28

### Message Hashing

The contract uses Ethereum's message prefix:
```
"\x19Ethereum Signed Message:\n32" + messageHash
```

## Testing

Run the test suite:

```bash
# Run all tests
forge test --match-contract SimpleICPSignatureVerifierTest

# Run specific test
forge test --match-test testWithdrawWithValidSignature

# Run with gas reporting
forge test --match-contract SimpleICPSignatureVerifierTest --gas-report
```

### Test Coverage

- ✅ Contract initialization and configuration
- ✅ ICP signature verification (valid and invalid)
- ✅ Fund deposit and withdrawal operations
- ✅ Signature replay protection
- ✅ Error handling and edge cases
- ✅ Access control and permissions
- ✅ Emergency functions

## Security Features

### Signature Validation
- All signatures validated against configured ICP canister address
- Invalid signatures rejected before state changes
- Signature format validation

### Replay Protection
- Each signature can only be used once
- Signature reuse tracked and prevented
- Nonce-based message hashing

### Access Control
- Only owner can update ICP canister address
- Only owner can transfer ownership
- Only owner can perform emergency withdrawals

## Gas Optimization

The simplified implementation is optimized for gas efficiency:

- **Minimal Storage**: Only essential state variables
- **Efficient Operations**: Optimized signature verification
- **No External Calls**: Zero external dependencies
- **Compact Functions**: Streamlined function implementations

## Deployment Options

### Single Instance
```bash
forge script script/DeploySimpleICPSignatureVerifier.s.sol --rpc-url $RPC_URL --broadcast
```

### Multiple Instances
```solidity
address[] memory canisters = [canister1, canister2, canister3];
deployMultiple(canisters, "ethereum");
```

### Custom Configuration
```solidity
deployAndVerify(icpCanister, chainId);
```

## Monitoring and Events

The contract emits events for monitoring:

```solidity
event FundsDeposited(string canisterId, address indexed account, uint256 amount, string chain);
event FundsWithdrawn(string canisterId, address indexed account, uint256 amount, string chain);
event ICPSignatureVerified(string canisterId, bytes32 messageHash, bool isValid);
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
event ICPCanisterUpdated(address indexed previousCanister, address indexed newCanister);
```

## Troubleshooting

### Common Issues

1. **Invalid Signature**: Check ICP canister private key and address derivation
2. **Signature Replay**: Ensure nonces are unique and signatures aren't reused
3. **Insufficient Balance**: Verify sufficient funds are deposited
4. **Access Denied**: Check caller has appropriate permissions

### Debug Functions

```solidity
// Check if signature was used
bool used = icpVerifier.isSignatureUsed(signature);

// Get contract information
(address owner, address icpCanister, string chainId, bool initialized) = icpVerifier.getContractInfo();

// Verify custom message
bool isValid = icpVerifier.verifyCustomMessage(message, signature);
```

## Comparison with Full Version

| Feature | Simple Version | Full Version |
|---------|---------------|--------------|
| Dependencies | None | OpenZeppelin |
| Token Support | Native ETH only | ETH + ERC20 |
| Upgradeability | No | Yes (UUPS) |
| Access Control | Basic | Advanced |
| Gas Cost | Lower | Higher |
| Complexity | Low | High |

## Contributing

When contributing to this example:

1. Follow the existing code style
2. Add comprehensive tests
3. Update documentation
4. Ensure gas efficiency
5. Maintain simplicity

## License

This example is provided under the same license as the main project.

## Support

For questions or issues:

1. Check the test suite for usage examples
2. Review the contract documentation
3. Examine the deployment scripts
4. Consult the main project documentation

