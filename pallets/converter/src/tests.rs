use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use xcm::opaque::latest::Junction::{GeneralKey, Parachain};
use xcm::opaque::latest::Junctions::X2;
use xcm::v1::MultiLocation;

#[test]
fn it_works_register_change_delete() {
	new_test_ext().execute_with(|| {
		// // Dispatch a signed extrinsic.
		// assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		// // Read pallet storage and assert an expected result.
		// assert_eq!(TemplateModule::something(), Some(42));
		let asset_id = [
			1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
			1, 1, 1,
		];
		let new_asset_id = [
			2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
			2, 2, 2,
		];
		let multilocation = MultiLocation::parent();
		let new_multilocation = MultiLocation {
			parents: 1,
			interior: X2(Parachain(666), GeneralKey(b"TEST_ASSET".to_vec())),
		};

		// Create:
		assert_ok!(Converter::register_mapping(Origin::root(), asset_id, multilocation.clone()));
		assert_eq!(
			Converter::get_multilocation_from_asset_id(asset_id)
				.expect("it_works_register_change_delete, Create: multilocation is None"),
			multilocation.clone()
		);
		assert_eq!(
			Converter::get_asset_id_from_multilocation(multilocation)
				.expect("it_works_register_change_delete, Create: asset id is None"),
			asset_id
		);

		// Change Asset's Multilocation:
		assert_ok!(Converter::change_asset_mapping(
			Origin::root(),
			asset_id,
			new_multilocation.clone()
		));
		assert_eq!(
			Converter::get_multilocation_from_asset_id(asset_id)
				.expect("it_works_register_change_delete, Change Asset's Multilocation: new_multilocation is None"),
			new_multilocation.clone()
		);
		assert_eq!(
			Converter::get_asset_id_from_multilocation(new_multilocation.clone()).expect(
				"it_works_register_change_delete, Change Asset's Multilocation: asset_id is None"
			),
			asset_id
		);

		todo!()
		// Change Multilocation's Asset
		// assert_ok!(Converter::change_multilocation_mapping(
		// 	Origin::root(),
		// 	new_multilocation.clone(),
		// 	new_asset_id,
		// ));

		// // Delete:
		// assert_ok!(Converter::delete_mapping(Origin::root(), asset_id, new_multilocation.clone()));
		// assert_eq!(Converter::get_multilocation_from_asset_id(asset_id), None);
		// assert_eq!(Converter::get_asset_id_from_multilocation(new_multilocation), None);
	});
}

#[test]
fn it_fails() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		// assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
		todo!();
	});
}
