cargo build --release --features runtime-benchmarks

./target/release/realis benchmark \
--chain dev \
--execution wasm \
--wasm-execution compiled \
--pallet pallet-nft-delegate \
--extrinsic '*' \
--steps 20 \
--repeat 10 \
--raw \
--output=./frame/nft-delegate/src/weights.rs \
--template=./.maintain/frame-weight-template.hbs