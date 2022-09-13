use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn should_init_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(BasicToken::init(Origin::signed(1)));
        assert_eq!(BasicToken::get_balance(1), 30000000);
    })
}

#[test]
fn cant_be_double_init() {
    new_test_ext().execute_with(|| {
        assert_ok!(BasicToken::init(Origin::signed(1)));
        assert_noop!(
			BasicToken::init(Origin::signed(1)),
			Error::<Test>::AlreadyInitialized
		);
    })
}

#[test]
fn should_transfer_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(BasicToken::init(Origin::signed(1)));

        // Transfer 100 tokens from user 1 to user 2
        assert_ok!(BasicToken::transfer(Origin::signed(1), 2, 100));

        assert_eq!(BasicToken::get_balance(1), 29999900);
        assert_eq!(BasicToken::get_balance(2), 100);
    })
}

#[test]
fn cant_spend_more_than_you_have() {
    new_test_ext().execute_with(|| {
        assert_ok!(BasicToken::init(Origin::signed(1)));
        assert_noop!(
			BasicToken::transfer(Origin::signed(1), 2, 30000001),
			Error::<Test>::InsufficientFunds
		);
    })
}