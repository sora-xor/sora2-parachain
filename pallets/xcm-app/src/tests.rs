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

use crate::{mock::*, Error};
use bridge_types::{types::AssetKind, H256};
use frame_support::{assert_err, assert_noop, assert_ok, traits::Currency};
use staging_xcm::{
    opaque::latest::{
        Junction::{GeneralKey, Parachain},
        Junctions::X2,
    },
    v3::MultiLocation,
};

#[test]
fn it_works_register_change_delete() {
    new_test_ext().execute_with(|| {
        let asset_id = [1; 32].into();
        let new_asset_id = [2; 32].into();
        let multilocation = MultiLocation::parent();
        let new_multilocation = MultiLocation {
            parents: 1,
            interior: X2(Parachain(666), GeneralKey { length: 6, data: test_general_key() }),
        };

        // Create:
        assert_ok!(XCMApp::register_mapping(asset_id, multilocation.clone()));
        assert_eq!(
            XCMApp::get_multilocation_from_asset_id::<H256>(asset_id.into())
                .expect("it_works_register_change_delete, Create: multilocation is None"),
            multilocation.clone()
        );
        assert_eq!(
            XCMApp::get_asset_id_from_multilocation(multilocation.clone())
                .expect("it_works_register_change_delete, Create: asset id is None"),
            asset_id
        );

        // Change Asset's Multilocation:
        assert_ok!(XCMApp::change_asset_mapping(asset_id, new_multilocation.clone()));
        assert_eq!(
			XCMApp::get_multilocation_from_asset_id(asset_id)
				.expect("it_works_register_change_delete, Change Asset's Multilocation: new_multilocation is None"),
			new_multilocation.clone()
		);
        assert_eq!(
            XCMApp::get_asset_id_from_multilocation(new_multilocation.clone()).expect(
                "it_works_register_change_delete, Change Asset's Multilocation: asset_id is None"
            ),
            asset_id
        );
        assert_eq!(XCMApp::get_asset_id_from_multilocation(multilocation.clone()), None);

        // Change Multilocation's Asset
        assert_ok!(XCMApp::change_multilocation_mapping(new_multilocation.clone(), new_asset_id,));
        assert_eq!(
			XCMApp::get_multilocation_from_asset_id(new_asset_id)
				.expect("it_works_register_change_delete, Change Multilocation's Asset: new_multilocation is None"),
			new_multilocation.clone()
		);
        assert_eq!(
            XCMApp::get_asset_id_from_multilocation(new_multilocation.clone()).expect(
                "it_works_register_change_delete, Change Multilocation's Asset: asset_id is None"
            ),
            new_asset_id
        );
        assert_eq!(XCMApp::get_multilocation_from_asset_id(asset_id), None);

        // Delete:
        assert_ok!(XCMApp::delete_mapping(new_asset_id));
        assert_eq!(XCMApp::get_multilocation_from_asset_id(new_asset_id), None);
        assert_eq!(XCMApp::get_asset_id_from_multilocation(new_multilocation), None);
    });
}

#[test]
fn it_fails_create_existing_multilocation_mapping() {
    new_test_ext().execute_with(|| {
        let asset_id = [1; 32].into();
        let multilocation = MultiLocation::parent();
        let new_multilocation = MultiLocation {
            parents: 1,
            interior: X2(Parachain(666), GeneralKey { length: 6, data: test_general_key() }),
        };

        assert_ok!(XCMApp::register_mapping(asset_id, multilocation.clone()));

        assert_noop!(
            XCMApp::register_mapping(asset_id, multilocation.clone()),
            Error::<Test>::MappingAlreadyExists
        );
        assert_noop!(
            XCMApp::register_mapping(asset_id, new_multilocation.clone()),
            Error::<Test>::MappingAlreadyExists
        );
    });
}

#[test]
fn it_fails_create_existing_asset_id_mapping() {
    new_test_ext().execute_with(|| {
        let asset_id = [1; 32].into();
        let new_asset_id = [2; 32].into();
        let multilocation = MultiLocation::parent();

        assert_ok!(XCMApp::register_mapping(asset_id, multilocation.clone()));

        assert_noop!(
            XCMApp::register_mapping(asset_id, multilocation.clone()),
            Error::<Test>::MappingAlreadyExists
        );
        assert_noop!(
            XCMApp::register_mapping(new_asset_id, multilocation.clone()),
            Error::<Test>::MappingAlreadyExists
        );
    });
}

#[test]
fn it_fails_change_asset_non_existing_mapping() {
    new_test_ext().execute_with(|| {
        let asset_id = [1; 32].into();
        let new_asset_id = [2; 32].into();
        let multilocation = MultiLocation::parent();

        assert_noop!(
            XCMApp::change_asset_mapping(asset_id, multilocation.clone()),
            Error::<Test>::MappingNotExist
        );

        assert_ok!(XCMApp::register_mapping(new_asset_id, multilocation.clone()));
        assert_noop!(
            XCMApp::change_asset_mapping(asset_id, multilocation.clone()),
            Error::<Test>::MappingNotExist
        );
        assert_eq!(
            XCMApp::get_asset_id_from_multilocation(multilocation.clone())
                .expect("it_fails_change_asset_non_existing_mapping: asset id is None"),
            new_asset_id
        );
    });
}

#[test]
fn it_fails_change_multilocation_non_existing_mapping() {
    new_test_ext().execute_with(|| {
        let asset_id = [1; 32].into();
        let multilocation = MultiLocation::parent();
        let new_multilocation = MultiLocation {
            parents: 1,
            interior: X2(Parachain(666), GeneralKey { length: 6, data: test_general_key() }),
        };

        assert_noop!(
            XCMApp::change_asset_mapping(asset_id, multilocation.clone()),
            Error::<Test>::MappingNotExist
        );

        assert_ok!(XCMApp::register_mapping(asset_id, new_multilocation.clone()));
        assert_noop!(
            XCMApp::change_multilocation_mapping(multilocation.clone(), asset_id),
            Error::<Test>::MappingNotExist
        );
        assert_eq!(
            XCMApp::get_multilocation_from_asset_id(asset_id)
                .expect("it_fails_change_multilocation_non_existing_mapping: asset id is None"),
            new_multilocation
        );
    });
}

#[test]
fn it_fails_delete_mapping_non_existing_mapping() {
    new_test_ext().execute_with(|| {
        let asset_id = [1; 32].into();
        assert_noop!(XCMApp::delete_mapping(asset_id), Error::<Test>::MappingNotExist);
    });
}

#[test]
fn it_works_register_asset() {
    new_test_ext().execute_with(|| {
        let asset_id = [1; 32].into();
        let multiasset = MultiLocation {
            parents: 1,
            interior: X2(Parachain(666), GeneralKey { length: 6, data: test_general_key() }),
        };
        assert_ok!(XCMApp::register_asset(
            RuntimeOrigin::root(),
            asset_id,
            multiasset.clone().into(),
            AssetKind::Sidechain,
            100000,
        ));
        assert_eq!(
            XCMApp::asset_minimum_amount(multiasset.clone()).expect("Min amount not set"),
            100000
        );
        assert_eq!(
            XCMApp::get_multilocation_from_asset_id::<H256>(asset_id.into())
                .expect("it_works_register_asset, Create: multilocation is None"),
            multiasset.clone()
        );
        assert_eq!(
            XCMApp::get_asset_id_from_multilocation(multiasset.clone())
                .expect("it_works_register_asset, Create: asset id is None"),
            asset_id
        );
        let new_asset_id = [2; 32].into();
        assert_noop!(
            XCMApp::register_asset(
                RuntimeOrigin::root(),
                new_asset_id,
                multiasset.clone().into(),
                AssetKind::Sidechain,
                100000,
            ),
            Error::<Test>::MappingAlreadyExists
        );
        assert_eq!(
            XCMApp::get_multilocation_from_asset_id::<H256>(asset_id.into())
                .expect("it_works_register_asset, Create: multilocation is None"),
            multiasset.clone()
        );
        assert_eq!(
            XCMApp::get_asset_id_from_multilocation(multiasset)
                .expect("it_works_register_asset, Create: asset id is None"),
            asset_id
        );
    });
}

#[test]
fn it_fails_send_xor_insufficient_balance() {
    new_test_ext().execute_with(|| {
        let _ = Balances::deposit_creating(&bob(), 10000000);
        assert_err!(
            XCMApp::send_xor_to_mainnet(RuntimeOrigin::signed(alice()), bob(), 1000000),
            pallet_balances::Error::<Test>::InsufficientBalance
        );
        assert_eq!(Balances::total_balance(&bob()), 10000000);
    });
}

#[test]
fn it_works_send_xor() {
    new_test_ext().execute_with(|| {
        let _ = Balances::deposit_creating(&bob(), 10000000);
        let _ = Balances::deposit_creating(&alice(), 10000000);
        assert_ok!(XCMApp::send_xor_to_mainnet(RuntimeOrigin::signed(alice()), bob(), 1000000),);
        assert_eq!(Balances::total_balance(&bob()), 10000000);
        assert_eq!(Balances::total_balance(&alice()), 9000000);
    });
}

#[test]
fn it_fails_receive_xor() {
    new_test_ext().execute_with(|| {
        let asset_id = [1; 32].into();
        assert_eq!(Balances::total_balance(&bob()), 0);
        assert_err!(
            XCMApp::xcm_transfer_asset(
                asset_id,
                alice(),
                staging_xcm::VersionedMultiLocation::V3(
                    staging_xcm::v3::MultiLocation::parent()
                        .pushed_with_interior(staging_xcm::v3::Junction::Parachain(2011))
                        .unwrap()
                        .pushed_with_interior(staging_xcm::v3::Junction::AccountId32 {
                            network: None,
                            id: bob().into()
                        })
                        .unwrap()
                ),
                1000000
            ),
            crate::Error::<Test>::InvalidAssetId
        );
        assert_eq!(Balances::total_balance(&bob()), 0);
        assert_err!(
            XCMApp::xcm_transfer_asset(
                asset_id,
                alice(),
                staging_xcm::VersionedMultiLocation::V2(
                    staging_xcm::v2::MultiLocation::parent()
                        .pushed_with_interior(staging_xcm::v2::Junction::Parachain(2011))
                        .unwrap()
                        .pushed_with_interior(staging_xcm::v2::Junction::AccountId32 {
                            network: staging_xcm::v2::NetworkId::Any,
                            id: bob().into()
                        })
                        .unwrap()
                ),
                1000000
            ),
            crate::Error::<Test>::WrongXCMVersion
        );
        assert_eq!(Balances::total_balance(&bob()), 0);
    });
}

#[test]
fn it_works_receive_xor() {
    new_test_ext().execute_with(|| {
        let asset_id = XorAssetId::get();
        assert_eq!(Balances::total_balance(&bob()), 0);
        assert_eq!(Balances::total_balance(&alice()), 0);
        assert_ok!(XCMApp::xcm_transfer_asset(
            asset_id,
            alice(),
            staging_xcm::VersionedMultiLocation::V3(
                staging_xcm::v3::MultiLocation::parent()
                    .pushed_with_interior(staging_xcm::v3::Junction::Parachain(2011))
                    .unwrap()
                    .pushed_with_interior(staging_xcm::v3::Junction::AccountId32 {
                        network: None,
                        id: bob().into()
                    })
                    .unwrap()
            ),
            1000000
        ),);
        assert_eq!(Balances::total_balance(&alice()), 0);
        assert_eq!(Balances::total_balance(&bob()), 1000000);
    });
}
