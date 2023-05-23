use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_claim() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

		/* -----
		   in mock.rs, AccountId is u64, so mock account is RuntimeOrigin::signed(1)

			 impl system::Config for Test {
				 type AccountId = u64;
			 }
		*/

		// test method call
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		// test get storage
		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	});
}

#[test]
fn revoke_claim() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));
	});
}

#[test]
fn transfer_claim() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);

		assert_ok!(
			PoeModule::transfer_claim(
				RuntimeOrigin::signed(1),
				2,
				claim.clone()
			)
		);

		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((2, frame_system::Pallet::<Test>::block_number()))
		);
	});
}

#[test]
fn create_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		// test error
		// assert_noop assets no chain state change
		assert_noop!(
			PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	});
}

#[test]
fn revoke_claim_non_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	});
}

#[test]
fn wrong_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	});
}