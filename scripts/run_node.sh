#!/bin/bash

./target/release/realis --dev --tmp --ws-port 9943  --rpc-port 9922  --validator  --rpc-methods=Unsafe  --listen-addr /ip4/0.0.0.0/tcp/30333 --name MyNode01 --unsafe-ws-external --unsafe-rpc-external --rpc-cors '*'