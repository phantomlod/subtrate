//! Benchmarking setup for pallet-Demo

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;


benchmarks! {
	create_kitty {
		// khởi tạo các tham số cho extrinsic benchmark
		let dnas : Vec<u8> = b"olala".to_vec();

		let caller: T::AccountId = whitelisted_caller();
	}: create_kitty (RawOrigin::Signed(caller), dnas)

	// kiểm tra lại trạng thái storage khi thực hiện extrinsic xem đúng chưa 
	verify {
		assert_eq!(KittyId::<T>::get(), 1);
	}

	transfer{
		let sender: T::AccountId = whitelisted_caller();
		let mut receiver: T::AccountId = account("receiver",0,0);
		let dna = vec![1,2,3];
		Kitties::<T>::create_kitty(RawOrigin::Signed(sender.clone()).into(),dna.clone() )?;
		let kitti_owner = Kitties::<T>::kitty_owned(sender.clone());
		let kitty = KittiesOwnedVec::<T>::get(&sender);
		let dna_hash = kitti_owner[0];


	}: transfer(RawOrigin::Signed(sender.clone()),receiver.clone(),dna_hash)

	verify {
		assert_eq!(KittiesOwnedVec::<T>::get(receiver.clone()), kitty);
	}
	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
	
}
