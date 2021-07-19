#!/bin/bash

# Build frames

#echo "Start build pallet-dynamic-fee"
#cargo build -p pallet-dynamic-fee
#echo "Start build pallet-ethereum"
#cargo build -p pallet-ethereum
#echo "Start build pallet-evm"
#cargo build -p pallet-evm

echo "Start build pallet-nft"
cargo build -p pallet-nft
echo "Start build realis-game-api"
cargo build -p realis-game-api
echo "Start build realis-game-api-rpc"
cargo build -p realis-game-api-rpc
echo "Start build pallet-staking"
cargo build -p pallet-staking

# Build primitives

#echo "Start build fp-consensus"
#cargo build -p fp-consensus
#echo "Start build fp-precompile"
#cargo build -p fp-precompile
echo "Start build realis-primitives"
cargo build -p realis-primitives
#echo "Start build fp-rpc"
#cargo build -p fp-rpc
#echo "Start build fp-storage"
#cargo build -p fp-storage

# Build

echo "Start build runtime-common"
cargo build -p runtime-common
echo "Start build node-runtime"
cargo build -p node-runtime
echo "Start build node-cli"
cargo build -p node-cli

# Build whole project

echo "Start build whole project"
cargo build --release