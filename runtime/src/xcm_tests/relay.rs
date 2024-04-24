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

use frame_support::{
    construct_runtime, parameter_types,
    traits::{ConstU128, ConstU32, ConstU64, Everything},
    weights::IdentityFee,
};
use frame_system::EnsureRoot;
use sp_core::H256;
use sp_runtime::{traits::IdentityLookup, AccountId32};
use staging_xcm::latest::Weight;
use xcm_simulator::{
    AggregateMessageOrigin, ProcessMessage, ProcessMessageError, UmpQueueId, WeightMeter,
};

use super::RelayNetwork;
use cumulus_primitives_core::ParaId;
use polkadot_runtime_parachains::{configuration, origin, shared};
use staging_xcm::latest::prelude::*;
use staging_xcm_builder::{
    AccountId32Aliases, AllowTopLevelPaidExecutionFrom, ChildParachainAsNative,
    ChildParachainConvertsVia, CurrencyAdapter as XcmCurrencyAdapter, FixedWeightBounds,
    IsConcrete, SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation,
    TakeWeightCredit, UsingComponents,
};
use staging_xcm_executor::{Config, XcmExecutor};

pub type AccountId = AccountId32;
pub type Balance = u128;

impl frame_system::Config for Runtime {
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Hash = H256;
    type Hashing = ::sp_runtime::traits::BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type BlockWeights = ();
    type BlockLength = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BaseCallFilter = Everything;
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
    type Nonce = u64;
    type Block = Block;
}

impl pallet_balances::Config for Runtime {
    type MaxLocks = ConstU32<50>;
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<1>;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxReserves = ConstU32<50>;
    type ReserveIdentifier = [u8; 8];
    type RuntimeHoldReason = ();
    type FreezeIdentifier = ();
    type MaxHolds = ();
    type MaxFreezes = ();
}

impl shared::Config for Runtime {}

impl configuration::Config for Runtime {
    type WeightInfo = configuration::TestWeightInfo;
}

parameter_types! {
    pub RelayLocation: MultiLocation = Here.into();
    pub Ancestry: MultiLocation = Here.into();
}

pub type SovereignAccountOf =
    (ChildParachainConvertsVia<ParaId, AccountId>, AccountId32Aliases<RelayNetwork, AccountId>);

pub type LocalAssetTransactor =
    XcmCurrencyAdapter<Balances, IsConcrete<RelayLocation>, SovereignAccountOf, AccountId, ()>;

type LocalOriginConverter = (
    SovereignSignedViaLocation<SovereignAccountOf, RuntimeOrigin>,
    ChildParachainAsNative<origin::Origin, RuntimeOrigin>,
    SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
);

pub type XcmRouter = super::RelayChainXcmRouter;
pub type Barrier = (TakeWeightCredit, AllowTopLevelPaidExecutionFrom<Everything>);

parameter_types! {
    pub Relay: MultiAssetFilter = Wild(AllOf { fun: WildFungible, id: Concrete(RelayLocation::get()) });
    pub const UnitWeightCost: Weight = Weight::from_parts(1_000_000_000, 0);
    pub const BaseXcmWeight: Weight = Weight::from_parts(100_000_000, 100_000_000);
    pub const MaxInstructions: u32 = 100;
    pub const MaxAssetsIntoHolding: u32 = 64;
    pub UniversalLocation: InteriorMultiLocation = X1(GlobalConsensus(RelayNetwork::get()));
}

pub struct XcmConfig;
impl Config for XcmConfig {
    type RuntimeCall = RuntimeCall;
    type XcmSender = XcmRouter;
    type AssetTransactor = LocalAssetTransactor;
    type OriginConverter = LocalOriginConverter;
    type IsReserve = ();
    type IsTeleporter = ();
    type UniversalLocation = UniversalLocation;
    type Barrier = Barrier;
    type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
    type Trader = UsingComponents<IdentityFee<Balance>, RelayLocation, AccountId, Balances, ()>;
    type ResponseHandler = ();
    type AssetTrap = ();
    type AssetClaims = ();
    type SubscriptionService = XcmPallet;
    type AssetLocker = XcmPallet;
    type AssetExchanger = ();
    type PalletInstancesInfo = ();
    type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
    type FeeManager = ();
    type MessageExporter = ();
    type UniversalAliases = frame_support::traits::Nothing;
    type CallDispatcher = RuntimeCall;
    type SafeCallFilter = Everything;
    type Aliasers = ();
}

pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

impl pallet_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type SendXcmOrigin = staging_xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
    type XcmRouter = XcmRouter;
    // Anyone can execute XCM messages locally...
    type ExecuteXcmOrigin =
        staging_xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
    type XcmExecuteFilter = Everything;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type XcmTeleportFilter = Everything;
    type XcmReserveTransferFilter = Everything;
    type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
    type UniversalLocation = UniversalLocation;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
    type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
    type Currency = Balances;
    type CurrencyMatcher = ();
    type TrustedLockers = ();
    type SovereignAccountOf = ();
    type MaxLockers = ConstU32<8>;
    type WeightInfo = pallet_xcm::TestWeightInfo;
    type AdminOrigin = EnsureRoot<AccountId>;
    type MaxRemoteLockConsumers = ();
    type RemoteLockConsumerIdentifier = ();
}

parameter_types! {
    /// Amount of weight that can be spent per block to service messages.
    pub MessageQueueServiceWeight: Weight = Weight::from_parts(1_000_000_000, 1_000_000);
    pub const MessageQueueHeapSize: u32 = 65_536;
    pub const MessageQueueMaxStale: u32 = 16;
}

/// Message processor to handle any messages that were enqueued into the `MessageQueue` pallet.
pub struct MessageProcessor;
impl ProcessMessage for MessageProcessor {
    type Origin = AggregateMessageOrigin;

    fn process_message(
        message: &[u8],
        origin: Self::Origin,
        meter: &mut WeightMeter,
        id: &mut [u8; 32],
    ) -> Result<bool, ProcessMessageError> {
        let para = match origin {
            AggregateMessageOrigin::Ump(UmpQueueId::Para(para)) => para,
        };
        staging_xcm_builder::ProcessXcmMessage::<
            Junction,
            staging_xcm_executor::XcmExecutor<XcmConfig>,
            RuntimeCall,
        >::process_message(message, Junction::Parachain(para.into()), meter, id)
    }
}

impl pallet_message_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Size = u32;
    type HeapSize = MessageQueueHeapSize;
    type MaxStale = MessageQueueMaxStale;
    type ServiceWeight = MessageQueueServiceWeight;
    type MessageProcessor = MessageProcessor;
    type QueueChangeHandler = ();
    type QueuePausedQuery = ();
    type WeightInfo = ();
}
impl origin::Config for Runtime {}

type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
    pub enum Runtime
    {
        System: frame_system,
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        ParasOrigin: origin::{Pallet, Origin},
        XcmPallet: pallet_xcm::{Pallet, Call, Storage, Event<T>, Origin},
        MessageQueue: pallet_message_queue::{Pallet, Event<T>},
    }
);
