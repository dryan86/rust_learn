//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as KittiesModule;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create() {
		let kitty_id = KittiesModule::<T>::get_next_id().unwrap();
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		create(RawOrigin::Signed(caller.clone()));

		let kitty = Kitties::<T>::get(kitty_id).unwrap();
		assert_last_event::<T>(Event::KittyCreated{who: caller.clone(), kitty_id, kitty}.into())
	}

	impl_benchmark_test_suite!(KittiesModule, crate::mock::new_test_ext(), crate::mock::Test);
}
