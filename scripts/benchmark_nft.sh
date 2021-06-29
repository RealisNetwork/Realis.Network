cargo build --release --features runtime-benchmarks

./target/release/realis benchmark \
--chain staging \
--execution wasm \
--wasm-execution compiled \
--pallet pallet_nft \
--extrinsic '*' \
--steps 20 \
--repeat 10 \
--raw \
--output=./frame/nft/src/weights.rs