cargo build --release --features runtime-benchmarks

./target/release/realis benchmark \
--chain dev \
--execution wasm \
--wasm-execution compiled \
--pallet realis-bridge \
--extrinsic '*' \
--steps 20 \
--repeat 10 \
--raw \
--output=./frame/realis-bridge/src/weights.rs \
--template=./.maintain/frame-weight-template.hbs