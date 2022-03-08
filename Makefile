.PHONY: init
init:
	./scripts/init.sh

.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check --release

.PHONY: test
test:
	SKIP_WASM_BUILD=1 cargo test --release --all

.PHONY: run
run:
	 ./target/release/realis --dev --tmp --ws-port 9943  --rpc-port 9922  --validator  --rpc-methods=Unsafe  --listen-addr /ip4/0.0.0.0/tcp/30333 --name MyNode01 --unsafe-ws-external --unsafe-rpc-external --rpc-cors '*'

.PHONY: build
build:
	SKIP_WASM_BUILD=1 cargo build --release

.PHONY: clean
clean:
	cd ../soul/nikita/chains/realis/ && rm -rf db && rm -rf network && cd ../../../danil/chains/realis/ && rm -rf db && rm -rf network && cd ../../../../Realis.Network

.PHONY: docker
docker:
	make build && cd target/release && mv realis ../../docker && cd ../../docker && bash ./run-realis.sh

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: clippy
clippy:
	cargo clippy
