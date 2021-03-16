#!/bin/bash
docker run -d --name=realis-test --net=host \
 -v /blockchain_soul/soul/nikita2:/realis/chain \
realis:1 \
/realis/realis \
--chain=realis \
--ws-port 9945  \
--rpc-port 9934  \
--validator  \
--rpc-methods=Unsafe  \
--reserved-nodes /ip4/144.91.101.91/tcp/30333/p2p/12D3KooWDPdX6yG1PpEUBJsoAZeo2wvhBo9GF3ZaGj4huA6xqGVC \
--name MyNode02 \
--unsafe-ws-external \
--unsafe-rpc-external \
--rpc-cors '*' \
-d /realis/chain
