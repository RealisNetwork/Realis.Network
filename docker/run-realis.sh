#!/bin/bash
IMAGE="daelon02/realis-network"
CONTAINER="realis-test"
docker rm -f ${CONTAINER}
docker rmi ${IMAGE}
docker build -t ${IMAGE} .
docker push ${IMAGE}
docker run -d --name=${CONTAINER} --net=host \
 -v /blockchain_soul/soul/nikita1:/realis/chain \
${IMAGE} \
/realis/realis \
--chain ../realis.json \
--ws-port 9944 \
--rpc-port 9933  \
--validator  \
--rpc-methods=Unsafe  \
--name MyNode01 \
--reserved-nodes /ip4/161.97.142.255/tcp/30333/p2p/12D3KooWPUArBJny2H3tsXvayd298WN4De9K6RW13eMmVceh5X3s \
--unsafe-ws-external \
--unsafe-rpc-external \
--rpc-cors '*' \
-d /realis/chain
