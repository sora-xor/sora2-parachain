#![cfg(test)]

use super::*;
use cumulus_primitives_core::ParaId;
use frame_support::{assert_err, assert_noop, assert_ok, traits::Currency};
use orml_traits::MultiCurrency;
use sp_runtime::{traits::AccountIdConversion, AccountId32};
use xcm_simulator::TestExt;

fn para_x_account() -> AccountId32 {
	ParaId::from(1).into_account_truncating()
}

fn sora_para_account() -> AccountId32 {
	ParaId::from(2).into_account_truncating()
}

// Not used in any unit tests, but it's super helpful for debugging. Let's
// keep it here. Don't forget to use  -- --nocapture when running test
// EXAMPLE: print_events::<crate::Runtime>("Transfer to SORA");
#[allow(dead_code)]
fn print_events<Runtime: frame_system::Config>(name: &'static str) {
	println!("------ {:?} events -------", name);
	frame_system::Pallet::<Runtime>::events()
		.iter()
		.for_each(|r| println!("> {:?}", r.event));
}

fn relay_native_asset_id() -> crate::H256 {
	hex_literal::hex!("54fd1e1728cd833d21da6f3e36c50884062e35edfc24aec7a70c18a60451255b").into()
}

fn prepeare_sora_parachain() {
	SoraParachain::execute_with(|| {
		let _ = SoraBalances::deposit_creating(&crate::GetTrustlessBridgeFeesAccountId::get(), 1000000000000000000);
		let _ = SoraBalances::deposit_creating(&ALICE, 1000000000000000000);
		let _ = SoraBalances::deposit_creating(&BOB, 1000000000000000000);
		assert_ok!(crate::XCMApp::register_mapping(
			crate::RuntimeOrigin::root(),
			relay_native_asset_id(),
			MultiLocation::new(1, Here)
		));
	});
}

#[test]
fn send_relay_chain_asset_to_sora_from_sibling() {
	TestNet::reset();

	Relay::execute_with(|| {
		let _ = RelayBalances::deposit_creating(&para_x_account(), 1000000000000000000);
	});

	prepeare_sora_parachain();

	ParaX::execute_with(|| {
		assert_ok!(ParaXTokens::transfer(
			Some(ALICE).into(),
			CurrencyId::R,
			1_000_000_000_00,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(2),
						Junction::AccountId32 { network: NetworkId::Any, id: BOB.into() }
					)
				)
				.into()
			),
			WeightLimit::Unlimited
		));
		assert_eq!(ParaTokens::free_balance(CurrencyId::R, &ALICE), 999999900000000000);
	});

	Relay::execute_with(|| {
		assert_eq!(RelayBalances::free_balance(&para_x_account()), 999999900000000000);
		assert_eq!(RelayBalances::free_balance(&sora_para_account()), 99999999960);
	});

	SoraParachain::execute_with(|| {
		assert!(frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| matches!(
			r.event,
			crate::RuntimeEvent::XCMApp(xcm_app::Event::AssetAddedToChannel(_))
		)));

		assert!(frame_system::Pallet::<crate::Runtime>::events()
			.iter()
			.any(|r| matches!(r.event, crate::RuntimeEvent::SubstrateBridgeOutboundChannel(_))));
	});
}

// #[test]
// fn send_relay_chain_asset_to_sibling() {
// 	TestNet::reset();

// 	Relay::execute_with(|| {
// 		// let _ = RelayBalances::deposit_creating(&para_x_account(), 1000000000000000000);
// 		let _ = RelayBalances::deposit_creating(&sora_para_account(), 1000000000000000000);
// 	});

// 	sora_register_native_relay_asset();

// 	SoraParachain::execute_with(|| {
// 		// assert_ok!(crate::XCMApp::test_xcm_transfer(
// 		assert_ok!(crate::XCMApp::test_xcm_transfer(
// 			crate::RuntimeOrigin::root(),
// 			relay_native_asset_id(),
// 			ALICE,
// 			xcm::VersionedMultiLocation::V1(MultiLocation::new(
// 				1,
// 				X2(Parachain(1), Junction::AccountId32 { network: NetworkId::Any, id: BOB.into() })
// 			)),
// 			10000000,
// 		));
// 	});

// 	Relay::execute_with(|| {
// 		print_events::<relay::Runtime>("!!!!! send_relay_chain_asset_to_sibling RELAY");
// 		// assert_eq!(RelayBalances::free_balance(&para_x_account()), 999999900000000000);

// 		// assert_eq!(RelayBalances::free_balance(&sora_para_account()), 99999999960);
// 	});

// 	ParaX::execute_with(|| {
// 		print_events::<para_x::Runtime>("!!!!! send_relay_chain_asset_to_sibling ParaX");
// 		assert_eq!(ParaTokens::free_balance(CurrencyId::R, &BOB), 500);
// 	});

// 	SoraParachain::execute_with(|| {
// 		print_events::<crate::Runtime>("!!!!! send_relay_chain_asset_to_sibling Sora");
// 	});

// }

// #[test]
// fn send_relay_chain_asset_to_relay_chain() {
// 	TestNet::reset();

// 	Relay::execute_with(|| {
// 		let _ = RelayBalances::deposit_creating(&sora_para_account(), 1_000_000_000_000_000);
// 	});

// 	sora_register_native_relay_asset();

// 	SoraParachain::execute_with(|| {
// 		assert_ok!(crate::XCMApp::test_xcm_transfer(
// 			crate::RuntimeOrigin::root(),
// 			relay_native_asset_id(),
// 			ALICE,
// 			xcm::VersionedMultiLocation::V1(MultiLocation::new(
// 				1,
// 				X1(Junction::AccountId32 { network: NetworkId::Any, id: ALICE.into() })
// 			)),
// 			1_000_000_000_000_000,
// 		));
// 		print_events::<crate::Runtime>("!!!!! send_relay_chain_asset_to_sibling Sora");
// 		// assert_eq!(ParaTokens::free_balance(CurrencyId::R, &ALICE), 500);
// 	});

// 	Relay::execute_with(|| {
// 		// assert_eq!(RelayBalances::free_balance(&sora_para_account()), 500);
// 		assert_eq!(RelayBalances::free_balance(&ALICE), 460);
// 	});
// }

#[test]
fn send_relay_chain_asset_to_sora_from_relay() {
	TestNet::reset();

	prepeare_sora_parachain();

	Relay::execute_with(|| {
		let _ = RelayBalances::deposit_creating(&ALICE, 1_000_000_000_000_000_000);
		// XcmPallet: pallet_xcm::{Pallet, Call, Storage, Event<T>, Origin},
		assert_ok!(relay::XcmPallet::reserve_transfer_assets(
			Some(ALICE).into(),
			Box::new(xcm::VersionedMultiLocation::V1(MultiLocation::new(
				0,
				X1(Junction::Parachain(2))
			))),
			Box::new(xcm::VersionedMultiLocation::V1(MultiLocation::new(
				0,
				X1(Junction::AccountId32 { network: NetworkId::Any, id: ALICE.into() })
			))),
			Box::new(xcm::VersionedMultiAssets::V1(
				vec![xcm::v1::MultiAsset {
					id: Concrete(MultiLocation::new(0, Here)),
					fun: Fungible(1_000_000_000_000_000),
				}]
				.into()
			)),
			0,
		));
	});

	SoraParachain::execute_with(|| {
		assert!(frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| matches!(
			r.event,
			crate::RuntimeEvent::XCMApp(xcm_app::Event::AssetAddedToChannel(_))
		)));

		assert!(frame_system::Pallet::<crate::Runtime>::events()
			.iter()
			.any(|r| matches!(r.event, crate::RuntimeEvent::SubstrateBridgeOutboundChannel(_))));
	});
}
