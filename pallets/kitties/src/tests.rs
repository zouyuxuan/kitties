use crate::{mock::*, Error, NextKittyId};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_create() {
    new_test_ext().execute_with(|| {
        assert_ok!(Kitties::create(Origin::signed(1)));
        assert_eq!(Kitties::next_kitty_id(), 1);

    });
}

#[test]
fn it_failed_for_create_when_maxkitties_created() {
    new_test_ext().execute_with(|| {
        NextKittyId::<Test>::put(u32::max_value());
        assert_noop!(Kitties::create(Origin::signed(1)), Error::<Test>::InvalidKittyId);
    });
}

#[test]
fn it_failed_for_create_when_reserve_not_enough() {
    new_test_ext().execute_with(|| {
        assert_noop!(Kitties::create(Origin::signed(3)), Error::<Test>::TokenNotEnough);
    });
}

#[test]
fn it_works_for_breed() {
    new_test_ext().execute_with(|| {
        let _ = Kitties::create(Origin::signed(1));
        let _ = Kitties::create(Origin::signed(1));
        let _ = Kitties::breed(Origin::signed(1), 0, 1);
        assert_eq!(Kitties::next_kitty_id(), 2);
    });
}

#[test]
fn it_failed_for_breed_with_same_id() {
    new_test_ext().execute_with(|| {
        let _ = Kitties::create(Origin::signed(1));
        assert_noop!(Kitties::breed(Origin::signed(1), 0, 0), Error::<Test>::SameKittyId);
    });
}

#[test]
fn it_failed_for_breed_id_invalid() {
    new_test_ext().execute_with(|| {
        assert_noop!(Kitties::breed(Origin::signed(1), 0, 1), Error::<Test>::InvalidKittyId);
    });
}

#[test]
fn it_failed_for_breed_max_kitties_created() {
    new_test_ext().execute_with(|| {
        let _ = Kitties::create(Origin::signed(1));
        let _ = Kitties::create(Origin::signed(2));
        NextKittyId::<Test>::put(u32::max_value());
        assert_noop!(Kitties::breed(Origin::signed(1), 0, 1), Error::<Test>::KittiesCountOverflow);
    });
}

#[test]
fn it_failed_for_breed_max_kitties_owned() {
    new_test_ext().execute_with(|| {
        let _ = Kitties::create(Origin::signed(1));
        let _ = Kitties::create(Origin::signed(1));
        assert_noop!(Kitties::breed(Origin::signed(1), 0, 1), Error::<Test>::ExceedMaxKittyOwned);
    });
}

#[test]
fn it_works_for_transfer() {
    new_test_ext().execute_with(|| {
        let _ = Kitties::create(Origin::signed(1));
        assert_ok!(Kitties::transfer(Origin::signed(1), 0, 2));
    });
}

#[test]
fn it_failed_for_transfer_when_kitty_not_exist() {
    new_test_ext().execute_with(|| {
        assert_noop!(Kitties::transfer(Origin::signed(1), 0, 2), Error::<Test>::InvalidKittyId);
    });
}

#[test]
fn it_failed_for_transfer_when_kitty_not_owner() {
    new_test_ext().execute_with(|| {
        let _ = Kitties::create(Origin::signed(1));
        assert_noop!(Kitties::transfer(Origin::signed(2), 0, 3), Error::<Test>::NotOwner);
    });
}

#[test]
fn it_failed_for_transfer_when_kitty_num_exceed() {
    new_test_ext().execute_with(|| {
        let _ = Kitties::create(Origin::signed(1));
        let _ = Kitties::create(Origin::signed(2));
        let _ = Kitties::create(Origin::signed(2));
        assert_noop!(Kitties::transfer(Origin::signed(1), 0, 2), Error::<Test>::ExceedMaxKittyOwned);
    });
}