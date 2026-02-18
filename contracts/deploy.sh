#!/bin/bash

set -e

echo "=== Deploying FvlRollup to local Anvil ==="

# Check anvil is running
if ! cast block latest --rpc-url http://localhost:8545 > /dev/null 2>&1; then
    echo "Anvil not running. Start it with: anvil"
    exit 1
fi

# Deploy contract using forge
# Anvil default deployer private key
PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
DEPLOYER="0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"

echo "Deploying from: $DEPLOYER"

# Deploy
DEPLOY_OUTPUT=$(forge create \
    --rpc-url http://localhost:8545 \
    --private-key $PRIVATE_KEY \
    contracts/FvlRollup.sol:FvlRollup \
    --broadcast 2>&1)

echo "$DEPLOY_OUTPUT"

# Extract contract address
CONTRACT_ADDRESS=$(echo "$DEPLOY_OUTPUT" | grep "Deployed to:" | awk '{print $3}')

if [ -z "$CONTRACT_ADDRESS" ]; then
    echo "Failed to extract contract address"
    exit 1
fi

echo ""
echo "Contract deployed at: $CONTRACT_ADDRESS"

# Write to data/contract.json
mkdir -p data
cat > data/contract.json << EOF
{
    "address": "$CONTRACT_ADDRESS",
    "deployer": "$DEPLOYER",
    "network": "local",
    "rpc_url": "http://localhost:8545"
}
EOF

echo "Contract address written to data/contract.json"
