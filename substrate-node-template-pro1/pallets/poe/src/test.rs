use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use sp_runtime::AccountId32;

#[test]
fn create_claim_works(){
    new_test_ext().execute_with(||{
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1, frame_system::Pallet::<Test>::block_number()))
        );
    });
}

#[test]
fn create_claim_failed_when_cliaim_already_exist(){
    new_test_ext().execute_with(||{
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_noop!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()), Error::<Test>::ProofAlreadyExist);
    });
}

#[test]
fn revoke_claim_works(){
    new_test_ext().execute_with(||{
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));
    });
}

#[test]
fn revoke_claim_failed_when_claim_not_exist(){
    new_test_ext().execute_with(||{
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn revoke_claim_failed_with_wrong_owner(){
    new_test_ext().execute_and_prove(||{
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    });
}

// transfer ok
#[test]
fn transfer_claim_works(){
    new_test_ext().execute_with(||{
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2));
    });
}

// transfer failed not exist
#[test]
fn transfer_claim_failed_when_claim_not_exist(){
    new_test_ext().execute_with(||{
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2),
            Error::<Test>::ClaimNotExist
        );
    })
}

// transfer failed not owner
#[test]
fn transfer_claim_failed_with_wrong_owner(){
    new_test_ext().execute_and_prove(||{
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(2), claim.clone(), 1),
            Error::<Test>::NotClaimOwner
        );
    });
}

