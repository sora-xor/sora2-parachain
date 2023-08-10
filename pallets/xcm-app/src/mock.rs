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

use crate as xcm_app;
use bridge_types::{traits::OutboundChannel, SubNetworkId};
use frame_support::{parameter_types, traits::Everything};
use frame_system as system;
use orml_traits::XcmTransfer;
use parachain_common::primitives::AssetId;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use xcm::latest::prelude::*;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

type AccountId = u128;
type Balance = u128;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        XCMApp: xcm_app::{Pallet, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl xcm_app::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type Balance = Balance;
    type OutboundChannel = TestOutboundChannel;
    type AccountIdToMultiLocation = TestAccountIdToMultiLocation;
    type XcmTransfer = TestXcmTransfer;
    type CallOrigin = TestCallOrigin;
    type AccountIdConverter = TestAccountIdConverter;
    type BalanceConverter = ();
    type XcmSender = ();
}

pub struct TestAccountIdConverter;
impl sp_runtime::traits::Convert<AccountId, sp_runtime::AccountId32> for TestAccountIdConverter {
    fn convert(a: AccountId) -> sp_runtime::AccountId32 {
        let b: [u8; 32] = [a.to_be_bytes(), a.to_be_bytes()].concat().try_into().unwrap();
        b.into()
    }
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

pub fn test_general_key() -> [u8; 32] {
    [3; 32]
}

pub struct TestOutboundChannel;

impl OutboundChannel<SubNetworkId, AccountId, ()> for TestOutboundChannel {
    fn submit(
        _network_id: SubNetworkId,
        _who: &system::RawOrigin<AccountId>,
        _payload: &[u8],
        _additional: (),
    ) -> Result<H256, sp_runtime::DispatchError> {
        Ok([1; 32].into())
    }
}

pub struct TestAccountIdToMultiLocation;
impl sp_runtime::traits::Convert<AccountId, MultiLocation> for TestAccountIdToMultiLocation {
    fn convert(account: AccountId) -> MultiLocation {
        let arr: [u8; 16] = account.to_be_bytes();
        let arrarr: [u8; 32] = [arr, arr]
            .concat()
            .try_into()
            .expect("Failed to convert account if to xcm multilocaton");
        X1(AccountId32 { network: Some(xcm::v3::NetworkId::Rococo), id: arrarr.into() }).into()
    }
}

pub struct TestXcmTransfer;
impl XcmTransfer<AccountId, Balance, AssetId> for TestXcmTransfer {
    fn transfer_multiasset(
        sender: AccountId,
        _asset: MultiAsset,
        dest: MultiLocation,
        _dest_weight_limit: WeightLimit,
    ) -> Result<orml_traits::xcm_transfer::Transferred<AccountId>, sp_runtime::DispatchError> {
        Ok(orml_traits::xcm_transfer::Transferred {
            sender,
            dest,
            assets: vec![].into(),
            fee: MultiAsset { id: Concrete(dest), fun: Fungible(0) },
        })
    }

    fn transfer_with_fee(
        sender: AccountId,
        _currency_id: AssetId,
        _amount: Balance,
        _fee: Balance,
        dest: MultiLocation,
        _dest_weight_limit: WeightLimit,
    ) -> Result<orml_traits::xcm_transfer::Transferred<AccountId>, sp_runtime::DispatchError> {
        Ok(orml_traits::xcm_transfer::Transferred {
            sender,
            dest,
            assets: vec![].into(),
            fee: MultiAsset { id: Concrete(dest), fun: Fungible(0) },
        })
    }

    fn transfer_multiasset_with_fee(
        sender: AccountId,
        _asset: MultiAsset,
        fee: MultiAsset,
        dest: MultiLocation,
        _dest_weight_limit: WeightLimit,
    ) -> Result<orml_traits::xcm_transfer::Transferred<AccountId>, sp_runtime::DispatchError> {
        Ok(orml_traits::xcm_transfer::Transferred { sender, dest, assets: vec![].into(), fee })
    }

    fn transfer_multicurrencies(
        sender: AccountId,
        _currencies: Vec<(AssetId, Balance)>,
        _fee_item: u32,
        dest: MultiLocation,
        _dest_weight_limit: WeightLimit,
    ) -> Result<orml_traits::xcm_transfer::Transferred<AccountId>, sp_runtime::DispatchError> {
        Ok(orml_traits::xcm_transfer::Transferred {
            sender,
            dest,
            assets: vec![].into(),
            fee: MultiAsset { id: Concrete(dest), fun: Fungible(0) },
        })
    }

    fn transfer_multiassets(
        sender: AccountId,
        assets: MultiAssets,
        fee: MultiAsset,
        dest: MultiLocation,
        _dest_weight_limit: WeightLimit,
    ) -> Result<orml_traits::xcm_transfer::Transferred<AccountId>, sp_runtime::DispatchError> {
        Ok(orml_traits::xcm_transfer::Transferred { sender, dest, assets, fee })
    }

    fn transfer(
        sender: AccountId,
        _currency_id: AssetId,
        _amount: Balance,
        dest: MultiLocation,
        _dest_weight_limit: WeightLimit,
    ) -> Result<orml_traits::xcm_transfer::Transferred<AccountId>, sp_runtime::DispatchError> {
        Ok(orml_traits::xcm_transfer::Transferred {
            sender,
            dest,
            assets: vec![].into(),
            fee: MultiAsset { id: Concrete(dest), fun: Fungible(0) },
        })
    }
}

pub struct TestCallOrigin;
impl<OuterOrigin: Default> frame_support::traits::EnsureOrigin<OuterOrigin> for TestCallOrigin {
    type Success = bridge_types::types::CallOriginOutput<SubNetworkId, H256, ()>;

    fn try_origin(_o: OuterOrigin) -> Result<Self::Success, OuterOrigin> {
        Ok(bridge_types::types::CallOriginOutput {
            network_id: SubNetworkId::Mainnet,
            message_id: [1; 32].into(),
            timepoint: bridge_types::GenericTimepoint::Sora(1),
            additional: (),
        })
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn try_successful_origin() -> Result<OuterOrigin, ()> {
        Ok(Default::default())
    }
}

impl Default for RuntimeOrigin {
    fn default() -> Self {
        RuntimeOrigin::root()
    }
}
