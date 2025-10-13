# ICP Signature Verification Examples - Summary

This directory contains two complete implementations of ICP signature verification in Ethereum smart contracts, converted from the original EVM Locker contract.

## Implementations

### 1. Full-Featured Version (`ICPSignatureVerifier.sol`)

**Features:**
- Complete OpenZeppelin integration (upgradeable, ownable, reentrancy guard)
- Support for both native ETH and ERC20 tokens
- Advanced access control and upgradeability
- Comprehensive error handling and events
- Full test suite with mock ERC20 token

**Files:**
- `ICPSignatureVerifier.sol` - Main contract
- `ICPSignatureVerifier.sol` (utils) - Signature verification library
- `ICPSignatureVerifier.t.sol` - Comprehensive test suite
- `DeployICPSignatureVerifier.s.sol` - Deployment script
- `README.md` - Detailed documentation

**Best for:** Production deployments requiring full feature set and upgradeability

### 2. Simplified Version (`SimpleICPSignatureVerifier.sol`)

**Features:**
- Zero external dependencies
- Native ETH support only
- Basic ownership model
- Gas-optimized implementation
- Easy to understand and deploy

**Files:**
- `SimpleICPSignatureVerifier.sol` - Main contract
- `SimpleICPSignatureVerifier.t.sol` - Test suite
- `DeploySimpleICPSignatureVerifier.s.sol` - Deployment script
- `SimpleREADME.md` - Documentation

**Best for:** Learning, prototyping, and simple use cases

## Key Differences from Original Locker

| Aspect | Original Locker | ICP Signature Verifier |
|--------|----------------|----------------------|
| **Signature Type** | EVM signatures | ICP signatures |
| **Signer** | Ethereum address | ICP canister address |
| **Verification** | `ecrecover` | Custom ICP verification |
| **Integration** | Direct EVM | Cross-chain (ICP ↔ EVM) |
| **Message Format** | Standard EVM | Ethereum-compatible |

## Core Functionality

Both implementations provide:

1. **Fund Management**
   - Deposit native tokens for ICP canisters
   - Withdraw with ICP signature verification
   - Balance tracking per canister

2. **Signature Verification**
   - Verify ICP signatures against canister addresses
   - Support for custom message verification
   - Ethereum-compatible message hashing

3. **Security Features**
   - Signature replay protection
   - Nonce-based message validation
   - Access control and permissions

4. **Monitoring**
   - Comprehensive event logging
   - Signature verification transparency
   - Contract state queries

## Usage Patterns

### Basic Workflow

1. **Deploy Contract**: Initialize with ICP canister address
2. **Deposit Funds**: Users deposit ETH for specific canisters
3. **ICP Signing**: ICP canister creates signatures for withdrawals
4. **Withdraw Funds**: Users withdraw with valid ICP signatures
5. **Monitor Events**: Track all operations via events

### Signature Creation (ICP Side)

```rust
// In your ICP canister
let message_hash = keccak256(abi.encodePacked(
    nonce, amount, user_address, chain_id, canister_id
));
let signature = ecdsa_sign(message_hash, private_key);
```

### Signature Verification (EVM Side)

```solidity
// In the smart contract
bytes32 messageHash = keccak256(abi.encodePacked(
    nonce, amount, msg.sender, chainId, canisterId
));
bool isValid = verifyICPSignature(messageHash, signature);
```

## Testing

Both implementations include comprehensive test suites:

```bash
# Full version tests
forge test --match-contract ICPSignatureVerifierTest

# Simple version tests
forge test --match-contract SimpleICPSignatureVerifierTest

# Run with gas reporting
forge test --match-contract ICPSignatureVerifierTest --gas-report
```

## Deployment

### Full Version
```bash
export ICP_CANISTER_ADDRESS="0x..."
export CHAIN_ID="ethereum"
forge script script/DeployICPSignatureVerifier.s.sol --rpc-url $RPC_URL --broadcast
```

### Simple Version
```bash
export ICP_CANISTER_ADDRESS="0x..."
export CHAIN_ID="ethereum"
forge script script/DeploySimpleICPSignatureVerifier.s.sol --rpc-url $RPC_URL --broadcast
```

## Security Considerations

### Signature Validation
- All signatures validated against configured ICP canister address
- Invalid signatures rejected before any state changes
- Signature format validation (65 bytes, valid recovery ID)

### Replay Protection
- Each signature can only be used once
- Signature reuse tracked and prevented
- Nonce-based message hashing prevents replay attacks

### Access Control
- Contract owner controls ICP canister address updates
- Users can only withdraw their own deposited funds
- Emergency withdrawal function for contract owner

## Integration Requirements

### ICP Canister Setup
1. Generate ECDSA key pair (secp256k1)
2. Derive Ethereum address from public key
3. Implement signature creation for withdrawal authorization
4. Configure canister to sign withdrawal requests

### Ethereum Contract Setup
1. Deploy contract with derived ICP canister address
2. Configure chain ID for message validation
3. Set up monitoring for signature verification events
4. Implement frontend integration for user interactions

## Monitoring and Events

Both contracts emit events for monitoring:

- `FundsDeposited`: When users deposit funds
- `FundsWithdrawn`: When users withdraw funds
- `ICPSignatureVerified`: When signatures are verified
- `OwnershipTransferred`: When ownership changes
- `ICPCanisterUpdated`: When canister address changes

## Gas Optimization

### Full Version
- Uses OpenZeppelin's optimized implementations
- Includes upgradeability overhead
- Supports multiple token types

### Simple Version
- Minimal storage and operations
- No external dependencies
- Optimized for gas efficiency

## Future Enhancements

Potential improvements for both implementations:

1. **Multi-signature Support**: Support for multiple ICP canisters
2. **Batch Operations**: Batch multiple withdrawals in single transaction
3. **Time-based Expiration**: Add time limits for signatures
4. **Advanced Access Control**: Role-based permissions
5. **Cross-chain Integration**: Support for multiple chains

## Contributing

When contributing to these examples:

1. Follow existing code style and patterns
2. Add comprehensive tests for new functionality
3. Update documentation for any changes
4. Ensure backward compatibility
5. Consider both full and simple versions

## Support

For questions or issues:

1. Review the comprehensive test suites
2. Check the detailed documentation
3. Examine the deployment scripts
4. Consult the main project documentation
5. Test with the provided examples

## License

These examples are provided under the same license as the main project. See the project root for license details.

