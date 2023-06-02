use crate::{Error,mock::*, Kitty};
use frame_support::{assert_ok,assert_noop};

#[test]
fn it_works_for_create(){
    new_test_ext().execute_with(||{
        let kitty_id = 0;
        let account_id = 1;
        
        assert_eq!(KittiesModule::next_kitty_id(),kitty_id);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

        assert_eq!(KittiesModule::next_kitty_id(),kitty_id+1);
        assert_eq!(KittiesModule::kitties(kitty_id).is_some(),true);
        assert_eq!(KittiesModule::kitty_owner(kitty_id),Some(account_id));
        assert_eq!(KittiesModule::kitty_parents(kitty_id),None);

        crate::NextKittyID::<Test>::set(crate::KittyID::max_value());
        assert_noop!(
            KittiesModule::create(RuntimeOrigin::signed(account_id)),
            Error::<Test>::InvalidKittyId
        );
    });
}

#[test]
fn it_works_for_breed(){
    new_test_ext().execute_with(||{
        let kitty_id = 0;
        let account_id = 1;

        assert_noop!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id),kitty_id,kitty_id),
            Error::<Test>::SameKittyId
        );

        assert_noop!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id),kitty_id+1,kitty_id+2),
            Error::<Test>::InvalidKittyId
        );

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

        assert_eq!(KittiesModule::next_kitty_id(),kitty_id+2);

        assert_ok!(KittiesModule::breed(RuntimeOrigin::signed(account_id),kitty_id,kitty_id+1));

        let breed_kitty_id = 2;
        assert_eq!(KittiesModule::next_kitty_id(),breed_kitty_id+1);
        assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(),true);
        assert_eq!(KittiesModule::kitty_owner(breed_kitty_id),Some(account_id));
        assert_eq!(KittiesModule::kitty_parents(breed_kitty_id),Some((kitty_id,kitty_id+1)));
    });
}

#[test]
fn it_works_for_transfer(){
    new_test_ext().execute_with(||{
        let kitty_id = 0;
        let account_id = 1;
        let recipient = 2;

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
        assert_eq!(KittiesModule::kitty_owner(kitty_id),Some(account_id));
        
        assert_noop!(
            KittiesModule::transfer(RuntimeOrigin::signed(recipient),account_id,kitty_id),
            Error::<Test>::NotOwner
        );

        assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id),recipient,kitty_id));

        assert_eq!(KittiesModule::kitty_owner(kitty_id),Some(recipient));

        assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(recipient),account_id,kitty_id));

        assert_eq!(KittiesModule::kitty_owner(kitty_id),Some(account_id));

    });
}
