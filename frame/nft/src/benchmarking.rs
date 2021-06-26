#[cfg(feature = "runtime-benchmarks")]

mod benchmarking {
	use crate::{*};
	use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
	use frame_system::RawOrigin;

	benchmarks!{
	mint {
		// The upper bound can be increased for greater accuracy
		let token_id = 1 .. 1000;
	}: mint(
			RawOrigin::Root,
			token_id.next().unwrap(),
			U256([1, 0, 0, 0]),
			Rarity::Common,
			Socket::Head,
			Params {
				strength: 1,
				agility: 1,
				intelligence: 1
			}
		)
	}

	impl_benchmark_test_suite!(
	PalletModule,
	crate::mock::new_test_ext(vec![1, 2, 3]),
	crate::mock::Test);
}
