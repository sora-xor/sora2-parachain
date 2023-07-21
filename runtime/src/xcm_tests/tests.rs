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
use bridge_types::{substrate::SubstrateAppCall, GenericTimepoint, SubNetworkId, traits::OutboundChannel};
use cumulus_primitives_core::ParaId;
use frame_support::{assert_ok, traits::Currency};
use orml_traits::MultiCurrency;
use sp_runtime::{traits::AccountIdConversion, AccountId32};
use xcm_simulator::TestExt;

fn para_x_account() -> AccountId32 {
    ParaId::from(1).into_account_truncating()
}

fn sora_para_account() -> AccountId32 {
    ParaId::from(2).into_account_truncating()
}

const PARA_X_ASSET_MIN_AMOUNT: u128 = 5000000;
const RELAY_ASSET_MIN_AMOUNT: u128 = 10000000;

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
            relay_native_asset_id(),
            MultiLocation::new(1, Here)
        ));
        assert_ok!(crate::XCMApp::register_mapping(
            para_x_asset_id(),
            MultiLocation::new(
                1,
                X2(Parachain(1), GeneralKey { length: 32, data: para_x_general_key() })
            )
        ));

        let bridge_origin = dispatch::RawOrigin::new(bridge_types::types::CallOriginOutput {
            network_id: SubNetworkId::Mainnet,
            additional: (),
            message_id: message_id(),
            timepoint: GenericTimepoint::Sora(1),
        });

        assert_ok!(crate::XCMApp::set_asset_minimum_amount(
            bridge_origin.clone().into(),
            relay_native_asset_id(),
            RELAY_ASSET_MIN_AMOUNT,
        ));
        assert_ok!(crate::XCMApp::set_asset_minimum_amount(
            bridge_origin.into(),
            para_x_asset_id(),
            PARA_X_ASSET_MIN_AMOUNT,
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
    });

    SoraParachain::execute_with(|| {
        assert!(frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| r.event
            == crate::RuntimeEvent::XCMApp(xcm_app::Event::AssetAddedToChannel(
                SubstrateAppCall::Transfer {
                    asset_id: relay_native_asset_id(),
                    sender: None,
                    recipient: BOB,
                    /// the comission shall be taken on the relaychain
                    amount: 96000000000,
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

    let val_to_send = 666555666555666;

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
            val_to_send,
            Box::new(
                MultiLocation::new(
                    1,
                    X2(
                        Parachain(2),
                        Junction::AccountId32 { network: None, id: BOB.into() }
                    )
                )
                .into()
            ),
            WeightLimit::Unlimited
        ));
    });

    SoraParachain::execute_with(|| {
        assert!(frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| r.event
            == crate::RuntimeEvent::XCMApp(xcm_app::Event::AssetAddedToChannel(
                SubstrateAppCall::Transfer {
                    asset_id: para_x_asset_id(),
                    sender: None,
                    recipient: BOB,
                    amount: val_to_send,
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
                X1(Junction::AccountId32 { network: None, id: ALICE.into() })
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
        assert!(frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| r.event
            == crate::RuntimeEvent::XCMApp(xcm_app::Event::AssetAddedToChannel(
                SubstrateAppCall::Transfer {
                    asset_id: relay_native_asset_id(),
                    sender: None,
                    recipient: ALICE,
                    amount: 1_000_000_000_000_000,
                }
            ))));

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
            crate::RuntimeEvent::PolkadotXcm(pallet_xcm::Event::AssetsTrapped(_, _, _))
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

        // check that assets are not transferred
        assert!(!frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| matches!(
            r.clone().event,
            crate::RuntimeEvent::XCMApp(xcm_app::Event::AssetTransferred(_, _, _, _))
        )));
        assert!(frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| matches!(
            r.clone().event,
            crate::RuntimeEvent::XCMApp(xcm_app::Event::AssetRefundSent(_, _, _, _))
        )));
    });
}

#[test]
fn send_relay_chain_asset_to_sora_from_relay_not_enough_tokens() {
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
                    fun: Fungible(RELAY_ASSET_MIN_AMOUNT - 1),
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
    });
}

#[test]
fn send_relay_chain_asset_to_sora_from_relay_exact_enough_tokens() {
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
                    fun: Fungible(RELAY_ASSET_MIN_AMOUNT),
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
fn send_sibling_chain_asset_to_sibling_asset_trapped() {
    TestNet::reset();

    Relay::execute_with(|| {
        let _ = RelayBalances::deposit_creating(&sora_para_account(), 1000000000000000000);
    });

    prepare_sora_parachain();

    SoraParachain::execute_with(|| {
        let location = xcm::v2::MultiLocation::parent();
        let assetid = para_x_asset_id();

        // fill queue
        for _ in 0..crate::BridgeMaxMessagesPerCommit::get() {
            let _ = crate::SubstrateBridgeOutboundChannel::submit(
                SubNetworkId::Mainnet,
                &frame_system::RawOrigin::Root,
                &[],
                (),
            );
        }
        let amount = 10000000;
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
            xcm::VersionedMultiLocation::V2(location.clone()),
            10000000,
        ));
        assert!(!frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| matches!(
            r.clone().event,
            crate::RuntimeEvent::XCMApp(xcm_app::Event::AssetTransferred(_, _, _, _))
        )));
        assert!(frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| matches!(
            r.clone().event,
            crate::RuntimeEvent::XCMApp(xcm_app::Event::BridgeAssetTrapped(_, _, _, _, _))
        )));
        let mes = crate::XCMApp::bridge_asset_trap(1).expect("asset trap does not exist");
        assert_eq!(mes.amount, amount);
        assert_eq!(mes.asset_id, assetid);
        assert_eq!(mes.recipient, ALICE);
    });
}

#[test]
fn send_sibling_chain_asset_to_sora_asset_trapped() {
    TestNet::reset();

    Relay::execute_with(|| {
        let _ = RelayBalances::deposit_creating(&sora_para_account(), 1000000000000000000);
    });

    prepare_sora_parachain();

    SoraParachain::execute_with(|| {
        let assetid = para_x_asset_id();

        // fill queue
        for _ in 0..crate::BridgeMaxMessagesPerCommit::get() {
            let _ = crate::SubstrateBridgeOutboundChannel::submit(
                SubNetworkId::Mainnet,
                &frame_system::RawOrigin::Root,
                &[],
                (),
            );
        }
        let amount = 10000000;
        assert_ok!(crate::XCMApp::add_to_channel(            
            ALICE,
            assetid,
            10000000,
        ));
        assert!(!frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| matches!(
            r.clone().event,
            crate::RuntimeEvent::XCMApp(xcm_app::Event::AssetAddedToChannel(_))
        )));
        assert!(frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| matches!(
            r.clone().event,
            crate::RuntimeEvent::XCMApp(xcm_app::Event::BridgeAssetTrapped(_, _, _, _, _))
        )));
        let mes = crate::XCMApp::bridge_asset_trap(1).expect("asset trap does not exist");
        assert_eq!(mes.amount, amount);
        assert_eq!(mes.asset_id, assetid);
        assert_eq!(mes.recipient, ALICE);
    });
}

#[test]
fn claim_refund_bridge_asset_works() {
    TestNet::reset();

    Relay::execute_with(|| {
        let _ = RelayBalances::deposit_creating(&sora_para_account(), 1000000000000000000);
    });

    prepare_sora_parachain();

    SoraParachain::execute_with(|| {
        let assetid = para_x_asset_id();
        let amount = 10000000;
        crate::XCMApp::trap_asset(Some(message_id()), assetid, ALICE,  amount, true);
        assert_ok!(crate::XCMApp::try_claim_bridge_asset(crate::RuntimeOrigin::root(), 1));
        assert!(crate::XCMApp::bridge_asset_trap(0).is_none());
        assert!(frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| matches!(
            r.clone().event,
            crate::RuntimeEvent::XCMApp(xcm_app::Event::TrappedMessageRefundSent(_, _, _, _))
        )));
    });
}

#[test]
fn claim_bridge_asset_works() {
    TestNet::reset();

    Relay::execute_with(|| {
        let _ = RelayBalances::deposit_creating(&sora_para_account(), 1000000000000000000);
    });

    prepare_sora_parachain();

    SoraParachain::execute_with(|| {
        let assetid = para_x_asset_id();
        let amount = 10000000;
        crate::XCMApp::trap_asset(Some(message_id()), assetid, ALICE,  amount, false);
        assert_ok!(crate::XCMApp::try_claim_bridge_asset(crate::RuntimeOrigin::root(), 1));
        assert!(crate::XCMApp::bridge_asset_trap(0).is_none());
        assert!(!frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| matches!(
            r.clone().event,
            crate::RuntimeEvent::XCMApp(xcm_app::Event::TrappedMessageRefundSent(_, _, _, _))
        )));
        assert!(frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| matches!(
            r.clone().event,
            crate::RuntimeEvent::XCMApp(xcm_app::Event::TrappedMessageSent(_, _, _))
        )));
    });
}

#[test]
fn claim_bridge_asset_fails() {
    TestNet::reset();

    Relay::execute_with(|| {
        let _ = RelayBalances::deposit_creating(&sora_para_account(), 1000000000000000000);
    });

    prepare_sora_parachain();

    SoraParachain::execute_with(|| {
        let assetid = para_x_asset_id();
        let amount = 10000000;
        crate::XCMApp::trap_asset(Some(message_id()), assetid, ALICE, amount, true);
        
        // fill queue
        for _ in 0..crate::BridgeMaxMessagesPerCommit::get() {
            let _ = crate::SubstrateBridgeOutboundChannel::submit(
                SubNetworkId::Mainnet,
                &frame_system::RawOrigin::Root,
                &[],
                (),
            );
        }

        let _ = crate::XCMApp::try_claim_bridge_asset(crate::RuntimeOrigin::root(), 1);
        assert!(!frame_system::Pallet::<crate::Runtime>::events().iter().any(|r| matches!(
            r.clone().event,
            crate::RuntimeEvent::XCMApp(xcm_app::Event::TrappedMessageRefundSent(_, _, _, _))
        )));
        let mes = crate::XCMApp::bridge_asset_trap(1).expect("asset trap does not exist");
        assert_eq!(mes.amount, amount);
        assert_eq!(mes.asset_id, assetid);
        assert_eq!(mes.recipient, ALICE);
    });
}

#[test]
fn trap_asset_nonce_works() {
    TestNet::reset();

    prepare_sora_parachain();

    SoraParachain::execute_with(|| {
        let assetid = para_x_asset_id();
        let amount = 10000000;

        crate::XCMApp::trap_asset(Some(message_id()), assetid, ALICE,  amount, true);
        assert!(crate::XCMApp::bridge_asset_trap(1).is_some());

        crate::XCMApp::trap_asset(Some(message_id()), assetid, ALICE,  amount, true);
        assert!(crate::XCMApp::bridge_asset_trap(2).is_some());

        crate::XCMApp::trap_asset(Some(message_id()), assetid, ALICE,  amount, false);
        assert!(crate::XCMApp::bridge_asset_trap(3).is_some());
        assert!(crate::XCMApp::bridge_asset_trap(4).is_none());
    });
}
