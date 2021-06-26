#[cfg(feature = "runtime-benchmarks")]

mod benchmarking {
	use crate::{*};
	use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
	use frame_system::RawOrigin;
	use hex_literal::hex;

	// Get Alice AccountId
	fn alice<T: Config>() -> T::AccountId {
		let bytes = hex!("10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d");
		T::AccountId::decode(&mut &bytes[..]).unwrap_or_default()
	}

	benchmarks!{
		// TODO fix error: NotNftMaster
		// Name of function to benchmark
		mint {
			// The upper bound can be increased for greater accuracy
			let caller = alice::<T>();
			// Can be named only as letter
			let b in 1 .. 1000;
		}: _(
				RawOrigin::Signed(caller.clone()),
				alice::<T>(),
				U256([b.into(), 0, 0, 0]),
				Rarity::Common,
				Socket::Head,
				Params {
					strength: 1,
					agility: 1,
					intelligence: 1
				}
			)
		verify {
			assert_eq!(1, 1);
		}
	}

	impl_benchmark_test_suite!(
	PalletModule,
	crate::mock::new_test_ext(vec![1, 2, 3]),
	crate::mock::Test);
}
