// This file is part of the SORA network and Polkaswap app.

// Copyright (c) 2020, 2021, Polka Biome Ltd. All rights reserved.
// SPDX-License-Identifier: BSD-4-Clause

// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:

// Redistributions of source code must retain the above copyright notice, this list
// of conditions and the following disclaimer.
// Redistributions in binary form must reproduce the above copyright notice, this
// list of conditions and the following disclaimer in the documentation and/or other
// materials provided with the distribution.
//
// All advertising materials mentioning features or use of this software must display
// the following acknowledgement: This product includes software developed by Polka Biome
// Ltd., SORA, and Polkaswap.
//
// Neither the name of the Polka Biome Ltd. nor the names of its contributors may be used
// to endorse or promote products derived from this software without specific prior written permission.

// THIS SOFTWARE IS PROVIDED BY Polka Biome Ltd. AS IS AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL Polka Biome Ltd. BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
// BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
// OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use super::*;
use bridge_types::{substrate::SubstrateAppCall, GenericTimepoint, SubNetworkId};
use cumulus_primitives_core::ParaId;
use frame_support::{assert_noop, assert_ok, traits::Currency};
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

fn para_x_asset_id() -> crate::H256 {
    hex_literal::hex!("54fd1e1728cd833d21da6f3e36c50884062e35edfc24aec7a70c18a60451255a").into()
}

fn message_id() -> crate::H256 {
    hex_literal::hex!("54fd1e1728cd833d21da6f3e36c50884062e35edfc24aec7a70c18a60451255c").into()
}

fn prepare_sora_parachain() {
    SoraParachain::execute_with(|| {
        let _ = SoraBalances::deposit_creating(&ALICE, 1000000000000000000);
        let _ = SoraBalances::deposit_creating(&BOB, 1000000000000000000);
        assert_ok!(crate::XCMApp::register_mapping(
            crate::RuntimeOrigin::root(),
            relay_native_asset_id(),
            MultiLocation::new(1, Here)
        ));
        assert_ok!(crate::XCMApp::register_mapping(
            crate::RuntimeOrigin::root(),
            para_x_asset_id(),
            MultiLocation::new(
                1,
                X2(Parachain(1), GeneralKey { length: 32, data: para_x_general_key() })
            )
        ));
    });
}

#[test]
fn send_relay_chain_asset_to_sora_from_sibling() {
    TestNet::reset();

    Relay::execute_with(|| {
        let _ = RelayBalances::deposit_creating(&para_x_account(), 1000000000000000000);
    });

    prepare_sora_parachain();

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
                        Junction::AccountId32 { network: Some(NetworkId::Rococo), id: BOB.into() }
                    )
                )
                .into()
            ),
            WeightLimit::Unlimited
        ));
        assert_eq!(ParaTokens::free_balance(CurrencyId::R, &ALICE), 999999900000000000);
    });

    SoraParachain::execute_with(|| {
        assert!(frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| r.event ==
            crate::RuntimeEvent::XCMApp(xcm_app::Event::AssetAddedToChannel(
                SubstrateAppCall::Transfer {
                    asset_id: relay_native_asset_id(),
                    sender: None,
                    recipient: BOB,
                    amount: 92000000000,
                }
            ))));

        assert!(frame_system::Pallet::<crate::Runtime>::events()
            .iter()
            .any(|r| matches!(r.event, crate::RuntimeEvent::SubstrateBridgeOutboundChannel(_))));
    });
}

#[test]
fn send_sibling_asset_to_sora_from_sibling() {
    TestNet::reset();

    Relay::execute_with(|| {
        let _ = RelayBalances::deposit_creating(&para_x_account(), 1000000000000000000);
    });

    prepare_sora_parachain();

    ParaX::execute_with(|| {
        let _ = ParaTokens::set_balance(
            para_x::RuntimeOrigin::root(),
            ALICE,
            CurrencyId::X,
            999999999999999999999,
            0,
        );
        assert_ok!(ParaXTokens::transfer(
            Some(ALICE).into(),
            CurrencyId::X,
            10000000000000000,
            Box::new(
                MultiLocation::new(
                    1,
                    X2(
                        Parachain(2),
                        Junction::AccountId32 { network: Some(NetworkId::Rococo), id: BOB.into() }
                    )
                )
                .into()
            ),
            WeightLimit::Unlimited
        ));
        assert_eq!(ParaTokens::free_balance(CurrencyId::X, &ALICE), 999989999999999999999);
    });

    SoraParachain::execute_with(|| {
        assert!(frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| r.event ==
            crate::RuntimeEvent::XCMApp(xcm_app::Event::AssetAddedToChannel(
                SubstrateAppCall::Transfer {
                    asset_id: para_x_asset_id(),
                    sender: None,
                    recipient: BOB,
                    amount: 9999996000000000,
                }
            ))));

        assert!(frame_system::Pallet::<crate::Runtime>::events()
            .iter()
            .any(|r| matches!(r.event, crate::RuntimeEvent::SubstrateBridgeOutboundChannel(_))));
    });
}

#[test]
fn send_relay_chain_asset_to_sibling() {
    TestNet::reset();

    Relay::execute_with(|| {
        let _ = RelayBalances::deposit_creating(&sora_para_account(), 1000000000000000000);
    });

    prepare_sora_parachain();

    SoraParachain::execute_with(|| {
        let location = MultiLocation::new(
            1,
            X2(
                Parachain(1),
                Junction::AccountId32 { network: Some(NetworkId::Rococo), id: BOB.into() },
            ),
        );
        let assetid = relay_native_asset_id();
        assert_ok!(crate::XCMApp::transfer(
            dispatch::RawOrigin::new(bridge_types::types::CallOriginOutput {
                network_id: SubNetworkId::Mainnet,
                additional: (),
                message_id: message_id(),
                timepoint: GenericTimepoint::Sora(1),
            })
            .into(),
            assetid,
            ALICE,
            xcm::VersionedMultiLocation::V3(location.clone()),
            10000000,
        ));
        let test_event = crate::RuntimeEvent::XCMApp(xcm_app::Event::AssetTransferred(
            ALICE,
            location.clone(),
            assetid,
            10000000,
        ));
        assert!(frame_system::Pallet::<crate::Runtime>::events()
            .iter()
            .any(|r| r.clone().event == test_event));
    });
}

#[test]
fn send_sibling_chain_asset_to_sibling() {
    TestNet::reset();

    Relay::execute_with(|| {
        let _ = RelayBalances::deposit_creating(&sora_para_account(), 1000000000000000000);
    });

    prepare_sora_parachain();

    SoraParachain::execute_with(|| {
        let location = MultiLocation::new(
            1,
            X2(
                Parachain(1),
                Junction::AccountId32 { network: Some(NetworkId::Rococo), id: BOB.into() },
            ),
        );
        let assetid = para_x_asset_id();
        assert_ok!(crate::XCMApp::transfer(
            dispatch::RawOrigin::new(bridge_types::types::CallOriginOutput {
                network_id: SubNetworkId::Mainnet,
                additional: (),
                message_id: message_id(),
                timepoint: GenericTimepoint::Sora(1),
            })
            .into(),
            assetid,
            ALICE,
            xcm::VersionedMultiLocation::V3(location.clone()),
            10000000,
        ));
        let test_event = crate::RuntimeEvent::XCMApp(xcm_app::Event::AssetTransferred(
            ALICE, location, assetid, 10000000,
        ));
        assert!(frame_system::Pallet::<crate::Runtime>::events()
            .iter()
            .any(|r| r.clone().event == test_event));
    });
}

#[test]
fn send_relay_chain_asset_to_relay_chain() {
    TestNet::reset();

    Relay::execute_with(|| {
        let _ = RelayBalances::deposit_creating(&sora_para_account(), 1_000_000_000_000_000);
    });

    prepare_sora_parachain();

    SoraParachain::execute_with(|| {
        let location = MultiLocation::new(
            1,
            X1(Junction::AccountId32 { network: Some(NetworkId::Rococo), id: ALICE.into() }),
        );
        let assetid = relay_native_asset_id();
        assert_ok!(crate::XCMApp::transfer(
            dispatch::RawOrigin::new(bridge_types::types::CallOriginOutput {
                network_id: SubNetworkId::Mainnet,
                additional: (),
                message_id: message_id(),
                timepoint: GenericTimepoint::Sora(1),
            })
            .into(),
            assetid,
            ALICE,
            xcm::VersionedMultiLocation::V3(location.clone()),
            1_000_000_000_000_000,
        ));
        let test_event = crate::RuntimeEvent::XCMApp(xcm_app::Event::AssetTransferred(
            ALICE,
            location,
            assetid,
            1_000_000_000_000_000,
        ));
        assert!(frame_system::Pallet::<crate::Runtime>::events()
            .iter()
            .any(|r| r.clone().event == test_event));
    });
}

#[test]
fn send_relay_chain_asset_to_sora_from_relay() {
    TestNet::reset();

    prepare_sora_parachain();

    Relay::execute_with(|| {
        let _ = RelayBalances::deposit_creating(&ALICE, 1_000_000_000_000_000_000);
        assert_ok!(relay::XcmPallet::reserve_transfer_assets(
            Some(ALICE).into(),
            Box::new(xcm::VersionedMultiLocation::V3(MultiLocation::new(
                0,
                X1(Junction::Parachain(2))
            ))),
            Box::new(xcm::VersionedMultiLocation::V3(MultiLocation::new(
                0,
                X1(Junction::AccountId32 { network: Some(NetworkId::Rococo), id: ALICE.into() })
            ))),
            Box::new(xcm::VersionedMultiAssets::V3(
                vec![xcm::v3::MultiAsset {
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

#[test]
fn send_to_sora_no_mapping_error() {
    TestNet::reset();

    Relay::execute_with(|| {
        let _ = RelayBalances::deposit_creating(&para_x_account(), 1000000000000000000);
    });

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
                        Junction::AccountId32 { network: Some(NetworkId::Rococo), id: BOB.into() }
                    )
                )
                .into()
            ),
            WeightLimit::Unlimited
        ));
        assert_eq!(ParaTokens::free_balance(CurrencyId::R, &ALICE), 999999900000000000);
    });

    SoraParachain::execute_with(|| {
        assert!(!frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| matches!(
            r.event,
            crate::RuntimeEvent::XCMApp(xcm_app::Event::AssetAddedToChannel(_))
        )));

        assert!(!frame_system::Pallet::<crate::Runtime>::events()
            .iter()
            .any(|r| matches!(r.event, crate::RuntimeEvent::SubstrateBridgeOutboundChannel(_))));

        assert!(frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| matches!(
            r.event,
            crate::RuntimeEvent::DmpQueue(cumulus_pallet_dmp_queue::Event::ExecutedDownward {
                message_id: _,
                outcome: Outcome::Incomplete(_, xcm::v3::Error::FailedToTransactAsset(_)),
            })
        )));
    });
}

#[test]
fn send_from_sora_no_mapping_error() {
    TestNet::reset();

    Relay::execute_with(|| {
        let _ = RelayBalances::deposit_creating(&para_x_account(), 1000000000000000000);
    });

    SoraParachain::execute_with(|| {
        let location = MultiLocation::new(
            1,
            X2(
                Parachain(1),
                Junction::AccountId32 { network: Some(NetworkId::Rococo), id: BOB.into() },
            ),
        );
        let assetid = relay_native_asset_id();
        assert_noop!(
            crate::XCMApp::transfer(
                dispatch::RawOrigin::new(bridge_types::types::CallOriginOutput {
                    network_id: SubNetworkId::Mainnet,
                    additional: (),
                    message_id: message_id(),
                    timepoint: GenericTimepoint::Sora(1),
                })
                .into(),
                assetid,
                ALICE,
                xcm::VersionedMultiLocation::V3(location.clone()),
                10000000,
            ),
            orml_xtokens::Error::<crate::Runtime>::NotCrossChainTransferableCurrency,
        );

        // check that assets are not transferred
        assert!(!frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| matches!(
            r.clone().event,
            crate::RuntimeEvent::XCMApp(xcm_app::Event::AssetTransferred(_, _, _, _))
        )));
    });
}

#[test]
fn send_relay_chain_asset_to_sora_from_relay_asset_trapped() {
    TestNet::reset();

    prepare_sora_parachain();

    Relay::execute_with(|| {
        let _ = RelayBalances::deposit_creating(&ALICE, 1_000_000_000_000_000_000);
        assert_ok!(relay::XcmPallet::reserve_transfer_assets(
            Some(ALICE).into(),
            Box::new(xcm::VersionedMultiLocation::V3(MultiLocation::new(
                0,
                X1(Junction::Parachain(2))
            ))),
            Box::new(xcm::VersionedMultiLocation::V3(MultiLocation::new(
                0,
                X1(Junction::AccountId32 { network: Some(NetworkId::Rococo), id: ALICE.into() })
            ))),
            Box::new(xcm::VersionedMultiAssets::V3(
                vec![xcm::v3::MultiAsset {
                    id: Concrete(MultiLocation::new(0, Here)),
                    fun: Fungible(1),
                }]
                .into()
            )),
            0,
        ));
    });

    SoraParachain::execute_with(|| {
        assert!(!frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| matches!(
            r.event,
            crate::RuntimeEvent::XCMApp(xcm_app::Event::AssetAddedToChannel(_))
        )));

        assert!(!frame_system::Pallet::<crate::Runtime>::events()
            .iter()
            .any(|r| matches!(r.event, crate::RuntimeEvent::SubstrateBridgeOutboundChannel(_))));

        // check that asset are trapped
        assert!(frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| matches!(
            r.event,
            crate::RuntimeEvent::PolkadotXcm(pallet_xcm::Event::AssetsTrapped(_, _, _))
        )));
    });
}
