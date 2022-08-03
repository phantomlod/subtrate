use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Demo::create_student(Origin::signed(1), b"olala".to_vec() , 123));
		// // Read pallet storage and assert an expected result.
		// assert_eq!(Demo::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(Demo::create_student(Origin::signed(1), b"olala".to_vec() , 12), Error::<Test>::TooYoung);
	});
}
