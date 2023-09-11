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
#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(all(feature = "std", not(feature = "parachain-gen")))]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[cfg(test)]
mod xcm_tests;

mod migrations;
mod trader;
mod weights;
pub mod xcm_config;

use bridge_types::{GenericNetworkId, SubNetworkId};
use codec::{Decode, Encode};
use frame_support::{
    dispatch::{DispatchClass, DispatchInfo, Dispatchable, PostDispatchInfo},
    traits::Contains,
};
use scale_info::TypeInfo;
use smallvec::smallvec;
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata, H256};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, Keccak256, Verify},
    transaction_validity::{
        TransactionLongevity, TransactionPriority, TransactionSource, TransactionValidity,
    },
    ApplyExtrinsicResult, MultiSignature, RuntimeDebug,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use frame_support::{
    construct_runtime, parameter_types,
    traits::Everything,
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, WEIGHT_REF_TIME_PER_SECOND},
        ConstantMultiplier, Weight, WeightToFeeCoefficient, WeightToFeeCoefficients,
        WeightToFeePolynomial,
    },
    PalletId,
};
use frame_system::{
    limits::{BlockLength, BlockWeights},
    EnsureRoot,
};
pub use sp_beefy::crypto::AuthorityId as BeefyId;
use sp_beefy::mmr::MmrLeafVersion;
pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
pub use sp_runtime::{MultiAddress, Perbill, Permill};
use xcm_config::{XcmConfig, XcmOriginToTransactDispatchOrigin};

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

// Polkadot Imports
use frame_support::weights::constants::RocksDbWeight;
use polkadot_runtime_common::{BlockHashCount, SlowAdjustingFeeUpdate};

// XCM Imports
use cumulus_pallet_parachain_system::RelayNumberStrictlyIncreases;
use xcm::latest::prelude::*;
use xcm_executor::XcmExecutor;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// An index to a block.
pub type BlockNumber = u32;

/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, ()>;

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;

/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
    generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra>;

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    migrations::Migrations,
>;

/// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
/// node's balance type.
///
/// This should typically create a mapping between the following ranges:
///   - `[0, MAXIMUM_BLOCK_WEIGHT]`
///   - `[Balance::min, Balance::max]`
///
/// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
///   - Setting it to `0` will essentially disable the weight fee.
///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
    type Balance = Balance;
    fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
        // in Rococo, extrinsic base weight (smallest non-zero weight) is mapped to 1 MILLIUNIT:
        // in our template, we map to 1/10 of that, or 1/10 MILLIUNIT
        let p = MILLIUNIT / 10;
        let q = 100 * Balance::from(ExtrinsicBaseWeight::get().ref_time());
        smallvec![WeightToFeeCoefficient {
            degree: 1,
            negative: false,
            coeff_frac: Perbill::from_rational(p % q, q),
            coeff_integer: p / q,
        }]
    }
}

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;
    use sp_runtime::{generic, traits::BlakeTwo256};

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;
}

impl_opaque_keys! {
    pub struct SessionKeys {
        pub aura: Aura,
        pub beefy: Beefy,
    }
}

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("sora_ksm"),
    impl_name: create_runtime_str!("sora_ksm"),
    authoring_version: 1,
    spec_version: 5,
    impl_version: 0,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 5,
    state_version: 1,
};

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 12000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

// Unit = the base number of indivisible units for balances
pub const UNIT: Balance = 1_000_000_000_000;
pub const MILLIUNIT: Balance = 1_000_000_000;
pub const MICROUNIT: Balance = 1_000_000;

/// The existential deposit. Set to 1/10 of the Connected Relay Chain.
pub const EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

/// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
/// used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);

/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
/// `Operational` extrinsics.
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

/// We allow for 0.5 of a second of compute with a 12 second average block time.
const MAXIMUM_BLOCK_WEIGHT: Weight = Weight::from_parts(
    WEIGHT_REF_TIME_PER_SECOND.saturating_div(2),
    cumulus_primitives_core::relay_chain::MAX_POV_SIZE as u64,
);

pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = HOURS;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

parameter_types! {
    pub const Version: RuntimeVersion = VERSION;

    // This part is copied from Substrate's `bin/node/runtime/src/lib.rs`.
    //  The `RuntimeBlockLength` and `RuntimeBlockWeights` exist here because the
    // `DeletionWeightLimit` and `DeletionQueueDepth` depend on those to parameterize
    // the lazy contract deletion.
    pub RuntimeBlockLength: BlockLength =
        BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
        .base_block(BlockExecutionWeight::get())
        .for_class(DispatchClass::all(), |weights| {
            weights.base_extrinsic = ExtrinsicBaseWeight::get();
        })
        .for_class(DispatchClass::Normal, |weights| {
            weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
        })
        .for_class(DispatchClass::Operational, |weights| {
            weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
            // Operational transactions have some extra reserved space, so that they
            // are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
            weights.reserved = Some(
                MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
            );
        })
        .avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
        .build_or_panic();
    pub const SS58Prefix: u16 = 420;
}

// Configure FRAME pallets to include in runtime.

impl frame_system::Config for Runtime {
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The aggregated dispatch type that is available for extrinsics.
    type RuntimeCall = RuntimeCall;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = AccountIdLookup<AccountId, ()>;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Index;
    /// The index type for blocks.
    type BlockNumber = BlockNumber;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The header type.
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    /// The ubiquitous origin type.
    type RuntimeOrigin = RuntimeOrigin;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// Runtime version.
    type Version = Version;
    /// Converts a module to an index of this module in the runtime.
    type PalletInfo = PalletInfo;
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// What to do if a new account is created.
    type OnNewAccount = ();
    /// What to do if an account is fully reaped from the system.
    type OnKilledAccount = ();
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// The basic call filter to use in dispatchable.
    type BaseCallFilter = Everything;
    /// Weight information for the extrinsics of this pallet.
    type SystemWeightInfo = ();
    /// Block & extrinsics weights: base values and limits.
    type BlockWeights = RuntimeBlockWeights;
    /// The maximum length of a block (in bytes).
    type BlockLength = RuntimeBlockLength;
    /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
    type SS58Prefix = SS58Prefix;
    /// The action to take on a Runtime Upgrade
    type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub const UncleGenerations: u32 = 0;
}

impl pallet_authorship::Config for Runtime {
    type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
    type EventHandler = (CollatorSelection,);
}

parameter_types! {
    pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
    type MaxLocks = MaxLocks;
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
}

parameter_types! {
    /// Relay Chain `TransactionByteFee` / 10
    pub const TransactionByteFee: Balance = 10 * MICROUNIT;
    pub const OperationalFeeMultiplier: u8 = 5;
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
    type WeightToFee = WeightToFee;
    type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
    type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
    type OperationalFeeMultiplier = OperationalFeeMultiplier;
}

parameter_types! {
    pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
    pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
}

impl cumulus_pallet_parachain_system::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnSystemEvent = ();
    type SelfParaId = parachain_info::Pallet<Runtime>;
    type DmpMessageHandler = DmpQueue;
    type ReservedDmpWeight = ReservedDmpWeight;
    type OutboundXcmpMessageSource = XcmpQueue;
    type XcmpMessageHandler = XcmpQueue;
    type ReservedXcmpWeight = ReservedXcmpWeight;
    type CheckAssociatedRelayNumber = RelayNumberStrictlyIncreases;
}

impl parachain_info::Config for Runtime {}

/// Configure Merkle Mountain Range pallet.
impl pallet_mmr::Config for Runtime {
    const INDEXING_PREFIX: &'static [u8] = b"mmr";
    type Hashing = Keccak256;
    type Hash = <Keccak256 as sp_runtime::traits::Hash>::Output;
    type OnNewRoot = pallet_beefy_mmr::DepositBeefyDigest<Runtime>;
    type WeightInfo = ();
    type LeafData = pallet_beefy_mmr::Pallet<Runtime>;
}

impl pallet_beefy::Config for Runtime {
    type BeefyId = BeefyId;
    type MaxAuthorities = MaxAuthorities;
    type OnNewValidatorSet = BeefyMmr;
}

parameter_types! {
    /// Version of the produced MMR leaf.
    ///
    /// The version consists of two parts;
    /// - `major` (3 bits)
    /// - `minor` (5 bits)
    ///
    /// `major` should be updated only if decoding the previous MMR Leaf format from the payload
    /// is not possible (i.e. backward incompatible change).
    /// `minor` should be updated if fields are added to the previous MMR Leaf, which given SCALE
    /// encoding does not prevent old leafs from being decoded.
    ///
    /// Hence we expect `major` to be changed really rarely (think never).
    /// See [`MmrLeafVersion`] type documentation for more details.
    pub LeafVersion: MmrLeafVersion = MmrLeafVersion::new(0, 0);
}

impl pallet_beefy_mmr::Config for Runtime {
    type LeafVersion = LeafVersion;
    type BeefyAuthorityToMerkleLeaf = pallet_beefy_mmr::BeefyEcdsaToEthereum;
    type LeafExtra = bridge_types::types::LeafExtraData<H256, H256>;
    type BeefyDataProvider = LeafProvider;
}

impl pallet_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
}

impl cumulus_pallet_aura_ext::Config for Runtime {}

#[cfg(not(test))]
impl cumulus_pallet_xcmp_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type ChannelInfo = ParachainSystem;
    type VersionWrapper = ();
    type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
    type ControllerOrigin = EnsureRoot<AccountId>;
    type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
    type WeightInfo = ();
    // TODO! look at this parameter
    type PriceForSiblingDelivery = ();
}

#[cfg(test)]
impl cumulus_pallet_xcmp_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type ChannelInfo = xcm_tests::ChannelInfo;
    type VersionWrapper = ();
    type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
    type ControllerOrigin = EnsureRoot<AccountId>;
    type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
    type WeightInfo = ();
    type PriceForSiblingDelivery = ();
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
}

parameter_types! {
    pub const Period: u32 = 6 * HOURS;
    pub const Offset: u32 = 0;
    pub const MaxAuthorities: u32 = 100_000;
}

impl pallet_session::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    // we don't have stash and controller, thus we don't need the convert as well.
    type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    type SessionManager = CollatorSelection;
    // Essentially just Aura, but lets be pedantic.
    type SessionHandler = <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
    type Keys = SessionKeys;
    type WeightInfo = ();
}

impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
    type DisabledValidators = ();
    type MaxAuthorities = MaxAuthorities;
}

parameter_types! {
    pub const PotId: PalletId = PalletId(*b"PotStake");
    pub const MaxCandidates: u32 = 1000;
    pub const MinCandidates: u32 = 5;
    pub const SessionLength: BlockNumber = 6 * HOURS;
    pub const MaxInvulnerables: u32 = 100;
    pub const ExecutiveBody: BodyId = BodyId::Executive;
}

// We allow root only to execute privileged collator selection operations.
pub type CollatorSelectionUpdateOrigin = EnsureRoot<AccountId>;

impl pallet_collator_selection::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type UpdateOrigin = CollatorSelectionUpdateOrigin;
    type PotId = PotId;
    type MaxCandidates = MaxCandidates;
    type MinCandidates = MinCandidates;
    type MaxInvulnerables = MaxInvulnerables;
    // should be a multiple of session or things will get inconsistent
    type KickThreshold = Period;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
    type ValidatorRegistration = Session;
    type WeightInfo = ();
}

impl xcm_app::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = xcm_app::weights::SubstrateWeight<Runtime>;
    type Balance = Balance;
    type OutboundChannel = SubstrateBridgeOutboundChannel;
    type AccountIdToMultiLocation = xcm_config::AccountIdToMultiLocation;
    type CallOrigin =
        dispatch::EnsureAccount<bridge_types::types::CallOriginOutput<SubNetworkId, H256, ()>>;
    type XcmTransfer = XTokens;
    type AccountIdConverter = sp_runtime::traits::Identity;
    type BalanceConverter = sp_runtime::traits::Identity;
    type XcmSender = XCMSenderWrapper;
}

pub struct XCMSenderWrapper;

impl xcm_app::XcmSender<Runtime> for XCMSenderWrapper {
    fn send_xcm(
        origin: frame_system::pallet_prelude::OriginFor<Runtime>,
        dest: Box<xcm::VersionedMultiLocation>,
        message: Box<xcm::VersionedXcm<()>>,
    ) -> frame_support::pallet_prelude::DispatchResult {
        PolkadotXcm::send(origin, dest, message)
    }
}

#[cfg(feature = "rococo")]
impl xcm_app_sudo_wrapper::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
    pub const SidechainRandomnessNetwork: SubNetworkId = SubNetworkId::Mainnet;
}

impl beefy_light_client::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Randomness = beefy_light_client::SidechainRandomness<Runtime, SidechainRandomnessNetwork>;
}

impl pallet_randomness_collective_flip::Config for Runtime {}

parameter_types! {
    pub const BridgeMaxMessagePayloadSize: u32 = 256;
    pub const BridgeMaxMessagesPerCommit: u32 = 20;
    pub const BridgeMaxTotalGasLimit: u64 = 5_000_000;
    pub const Decimals: u32 = 12;
}

impl dispatch::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OriginOutput = bridge_types::types::CallOriginOutput<SubNetworkId, H256, ()>;
    type Origin = RuntimeOrigin;
    type MessageId = bridge_types::types::MessageId;
    type Hashing = Keccak256;
    type Call = DispatchableSubstrateBridgeCall;
    type CallFilter = SubstrateBridgeCallFilter;
    type WeightInfo = dispatch::weights::SubstrateWeight<Runtime>;
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct DispatchableSubstrateBridgeCall(bridge_types::substrate::BridgeCall);

impl Dispatchable for DispatchableSubstrateBridgeCall {
    type RuntimeOrigin = crate::RuntimeOrigin;
    type Config = crate::Runtime;
    type Info = DispatchInfo;
    type PostInfo = PostDispatchInfo;

    fn dispatch(
        self,
        origin: Self::RuntimeOrigin,
    ) -> sp_runtime::DispatchResultWithInfo<Self::PostInfo> {
        match self.0 {
            bridge_types::substrate::BridgeCall::SubstrateApp(_msg) => Ok(().into()),
            bridge_types::substrate::BridgeCall::XCMApp(msg) => {
                let call: xcm_app::Call<crate::Runtime> = msg.into();
                let call: crate::RuntimeCall = call.into();
                call.dispatch(origin)
            },
            bridge_types::substrate::BridgeCall::DataSigner(msg) => {
                let call: bridge_data_signer::Call<crate::Runtime> = msg.into();
                let call: crate::RuntimeCall = call.into();
                call.dispatch(origin)
            },
            bridge_types::substrate::BridgeCall::MultisigVerifier(_) => Ok(().into()),
        }
    }
}

impl frame_support::dispatch::GetDispatchInfo for DispatchableSubstrateBridgeCall {
    fn get_dispatch_info(&self) -> DispatchInfo {
        match &self.0 {
            bridge_types::substrate::BridgeCall::SubstrateApp(_) => todo!(),
            bridge_types::substrate::BridgeCall::XCMApp(msg) => {
                let call: xcm_app::Call<crate::Runtime> = msg.clone().into();
                call.get_dispatch_info()
            },
            bridge_types::substrate::BridgeCall::DataSigner(msg) => {
                let call: bridge_data_signer::Call<crate::Runtime> = msg.clone().into();
                call.get_dispatch_info()
            },
            bridge_types::substrate::BridgeCall::MultisigVerifier(msg) => {
                let call: multisig_verifier::Call<crate::Runtime> = msg.clone().into();
                call.get_dispatch_info()
            },
        }
    }
}

pub struct SubstrateBridgeCallFilter;
impl Contains<DispatchableSubstrateBridgeCall> for SubstrateBridgeCallFilter {
    fn contains(call: &DispatchableSubstrateBridgeCall) -> bool {
        match &call.0 {
            bridge_types::substrate::BridgeCall::SubstrateApp(_) => false,
            bridge_types::substrate::BridgeCall::XCMApp(_) => true,
            bridge_types::substrate::BridgeCall::DataSigner(_) => true,
            bridge_types::substrate::BridgeCall::MultisigVerifier(_) => true,
        }
    }
}

#[cfg(feature = "rococo")]
parameter_types! {
    pub const ThisNetworkId: GenericNetworkId = GenericNetworkId::Sub(SubNetworkId::Rococo);
}

#[cfg(feature = "polkadot")]
parameter_types! {
    pub const ThisNetworkId: GenericNetworkId = GenericNetworkId::Sub(SubNetworkId::Polkadot);
}

#[cfg(feature = "kusama")]
parameter_types! {
    pub const ThisNetworkId: GenericNetworkId = GenericNetworkId::Sub(SubNetworkId::Kusama);
}

impl substrate_bridge_channel::inbound::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Verifier = MultisigVerifier;
    type MessageDispatch = SubstrateDispatch;
    type WeightInfo = substrate_bridge_channel::inbound::weights::SubstrateWeight<Runtime>;
    type UnsignedPriority = DataSignerPriority;
    type UnsignedLongevity = DataSignerLongevity;
    type MaxMessagePayloadSize = BridgeMaxMessagePayloadSize;
    type MaxMessagesPerCommit = BridgeMaxMessagesPerCommit;
    type ThisNetworkId = ThisNetworkId;
}

pub struct TimepointProvider;

impl bridge_types::traits::TimepointProvider for TimepointProvider {
    fn get_timepoint() -> bridge_types::GenericTimepoint {
        bridge_types::GenericTimepoint::Parachain(System::block_number())
    }
}

impl substrate_bridge_channel::outbound::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MessageStatusNotifier = ();
    type MaxMessagePayloadSize = BridgeMaxMessagePayloadSize;
    type MaxMessagesPerCommit = BridgeMaxMessagesPerCommit;
    type AuxiliaryDigestHandler = LeafProvider;
    type WeightInfo = ();
    type TimepointProvider = TimepointProvider;
    type ThisNetworkId = ThisNetworkId;
    type AssetId = ();
    type Balance = ();
}

impl leaf_provider::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Hashing = Keccak256;
    type Hash = <Keccak256 as sp_runtime::traits::Hash>::Output;
    type Randomness = beefy_light_client::SidechainRandomness<Runtime, SidechainRandomnessNetwork>;
}

parameter_types! {
    pub const BridgeMaxPeers: u32 = 50;
    // Not as important as some essential transactions (e.g. im_online or similar ones)
    pub DataSignerPriority: TransactionPriority = Perbill::from_percent(10) * TransactionPriority::max_value();
    // We don't want to have not relevant imports be stuck in transaction pool
    // for too long
    pub DataSignerLongevity: TransactionLongevity = EPOCH_DURATION_IN_BLOCKS as u64;
}

impl bridge_data_signer::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OutboundChannel = SubstrateBridgeOutboundChannel;
    type CallOrigin =
        dispatch::EnsureAccount<bridge_types::types::CallOriginOutput<SubNetworkId, H256, ()>>;
    type MaxPeers = BridgeMaxPeers;
    type UnsignedPriority = DataSignerPriority;
    type UnsignedLongevity = DataSignerLongevity;
    type WeightInfo = bridge_data_signer::weights::SubstrateWeight<Runtime>;
}

impl multisig_verifier::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type CallOrigin =
        dispatch::EnsureAccount<bridge_types::types::CallOriginOutput<SubNetworkId, H256, ()>>;
    type OutboundChannel = SubstrateBridgeOutboundChannel;
    type MaxPeers = BridgeMaxPeers;
    type WeightInfo = multisig_verifier::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const CouncilCollectiveMotionDuration: BlockNumber = 5 * DAYS;
    pub const CouncilCollectiveMaxProposals: u32 = 100;
    pub const CouncilCollectiveMaxMembers: u32 = 100;
}

// pub type CouncilCollective = pallet_collective::Instance1;
// pub type TechnicalCollective = pallet_collective::Instance2;

// type MoreThanHalfCouncil = EitherOfDiverse<
//     EnsureRoot<AccountId>,
//     pallet_collective::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>,
// >;
// type AtLeastHalfCouncil = EitherOfDiverse<
//     pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>,
//     EnsureRoot<AccountId>,
// >;
// type AtLeastTwoThirdsCouncil = EitherOfDiverse<
//     pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 2, 3>,
//     EnsureRoot<AccountId>,
// >;

// impl pallet_collective::Config<CouncilCollective> for Runtime {
//     type RuntimeOrigin = RuntimeOrigin;
//     type Proposal = RuntimeCall;
//     type RuntimeEvent = RuntimeEvent;
//     type MotionDuration = CouncilCollectiveMotionDuration;
//     type MaxProposals = CouncilCollectiveMaxProposals;
//     type MaxMembers = CouncilCollectiveMaxMembers;
//     type DefaultVote = pallet_collective::PrimeDefaultVote;
//     type WeightInfo = Lol;
// }

// pub struct Lol;
// // use frame_support::weights::Weight;
// // use sp_weights::weight_v2::Weight;

// impl pallet_collective::WeightInfo for Lol {
// 	fn set_members(m: u32, n: u32, p: u32, ) -> Weight {
//         todo!()
//     }
// 	fn execute(b: u32, m: u32, ) -> Weight{
//         todo!()
//     }
// 	fn propose_execute(b: u32, m: u32, ) -> Weight{
//         todo!()
//     }
// 	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight{
//         todo!()
//     }
// 	fn vote(m: u32, ) -> Weight{
//         todo!()
//     }
// 	fn close_early_disapproved(m: u32, p: u32, ) -> Weight{
//         todo!()
//     }
// 	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight{
//         todo!()
//     }
// 	fn close_disapproved(m: u32, p: u32, ) -> Weight{
//         todo!()
//     }
// 	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight{
//         todo!()
//     }
// 	fn disapprove_proposal(p: u32, ) -> Weight{
//         todo!()
//     }
// }

// impl pallet_collective::Config<TechnicalCollective> for Runtime {
//     type RuntimeOrigin = RuntimeOrigin;
//     type Proposal = RuntimeCall;
//     type RuntimeEvent = RuntimeEvent;
//     type MotionDuration = TechnicalCollectiveMotionDuration;
//     type MaxProposals = TechnicalCollectiveMaxProposals;
//     type MaxMembers = TechnicalCollectiveMaxMembers;
//     type DefaultVote = pallet_collective::PrimeDefaultVote;
//     type WeightInfo = CollectiveWeightInfo<Self>;
// }

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        // System support stuff.
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>} = 0,
        ParachainSystem: cumulus_pallet_parachain_system::{
            Pallet, Call, Config, Storage, Inherent, Event<T>, ValidateUnsigned,
        } = 1,
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent} = 2,
        ParachainInfo: parachain_info::{Pallet, Storage, Config} = 3,
        Mmr: pallet_mmr = 4,
        Beefy: pallet_beefy = 5,
        BeefyMmr: pallet_beefy_mmr = 6,

        // Monetary stuff.
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>} = 10,
        TransactionPayment: pallet_transaction_payment::{Pallet, Storage, Event<T>} = 11,

        // Collator support. The order of these 4 are important and shall not change.
        Authorship: pallet_authorship::{Pallet, Storage} = 20,
        CollatorSelection: pallet_collator_selection::{Pallet, Call, Storage, Event<T>, Config<T>} = 21,
        Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>} = 22,
        Aura: pallet_aura::{Pallet, Storage, Config<T>} = 23,
        AuraExt: cumulus_pallet_aura_ext::{Pallet, Storage, Config} = 24,

        // XCM helpers.
        XcmpQueue: cumulus_pallet_xcmp_queue::{Pallet, Call, Storage, Event<T>} = 30,
        PolkadotXcm: pallet_xcm::{Pallet, Event<T>, Origin, Config} = 31,
        CumulusXcm: cumulus_pallet_xcm::{Pallet, Event<T>, Origin} = 32,
        DmpQueue: cumulus_pallet_dmp_queue::{Pallet, Call, Storage, Event<T>} = 33,

        // ORML
        XTokens: orml_xtokens::{Pallet, Storage, Event<T>} = 41,

        Sudo: pallet_sudo::{Pallet, Call, Storage, Event<T>, Config<T>} = 100,

        XCMApp: xcm_app::{Pallet, Call, Storage, Event<T>} = 101,
        BeefyLightClient: beefy_light_client::{Pallet, Call, Storage, Event<T>, Config} = 103,
        SubstrateBridgeInboundChannel: substrate_bridge_channel::inbound::{Pallet, Call, Storage, Event<T>, ValidateUnsigned} = 104,
        SubstrateBridgeOutboundChannel: substrate_bridge_channel::outbound::{Pallet, Config<T>, Storage, Event<T>} = 105,
        SubstrateDispatch: dispatch::{Pallet, Storage, Event<T>, Origin<T>} = 106,
        LeafProvider: leaf_provider::{Pallet, Storage, Event<T>} = 107,
        BridgeDataSigner: bridge_data_signer::{Pallet, Storage, Event<T>, Call, ValidateUnsigned} = 108,
        MultisigVerifier: multisig_verifier::{Pallet, Storage, Event<T>, Call, Config} = 109,

		// Council: pallet_collective::<Instance1>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>} = 110,
		// Council: pallet_collective::<Instance1>::{Pallet, Call, Storage, Origin<T>, Event<T>} = 110,
		// TechnicalCommittee: pallet_collective::<Instance2>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>} = 111,

        #[cfg(feature = "rococo")]
        XCMAppSudoWrapper: xcm_app_sudo_wrapper::{Pallet, Call, Storage, Event<T>} = 150,
    }
);

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
    define_benchmarks!(
        [frame_system, SystemBench::<Runtime>]
        [pallet_balances, Balances]
        [pallet_session, SessionBench::<Runtime>]
        [pallet_timestamp, Timestamp]
        [pallet_collator_selection, CollatorSelection]
        [cumulus_pallet_xcmp_queue, XcmpQueue]
        [xcm_app, XCMApp]
        [pallet_xcm, PolkadotXcm]
    );
}

impl_runtime_apis! {
    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration {
            sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
        }

        fn authorities() -> Vec<AuraId> {
            Aura::authorities().into_inner()
        }
    }

    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block)
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl sp_beefy::BeefyApi<Block> for Runtime {
        fn validator_set() -> Option<sp_beefy::ValidatorSet<BeefyId>> {
            Beefy::validator_set()
        }
    }

    impl sp_mmr_primitives::MmrApi<Block, Hash, BlockNumber> for Runtime {
        fn mmr_root() -> Result<Hash, sp_mmr_primitives::Error> {
            Ok(Mmr::mmr_root())
        }

        fn mmr_leaf_count() -> Result<sp_mmr_primitives::LeafIndex, sp_mmr_primitives::Error> {
            Ok(Mmr::mmr_leaves())
        }

        fn generate_proof(
            block_numbers: Vec<BlockNumber>,
            best_known_block_number: Option<BlockNumber>,
        ) -> Result<(Vec<sp_mmr_primitives::EncodableOpaqueLeaf>, sp_mmr_primitives::Proof<Hash>), sp_mmr_primitives::Error> {
            Mmr::generate_proof(block_numbers, best_known_block_number).map(
                |(leaves, proof)| {
                    (
                        leaves
                            .into_iter()
                            .map(|leaf| sp_mmr_primitives::EncodableOpaqueLeaf::from_leaf(&leaf))
                            .collect(),
                        proof,
                    )
                },
            )
        }

        fn verify_proof(leaves: Vec<sp_mmr_primitives::EncodableOpaqueLeaf>, proof: sp_mmr_primitives::Proof<Hash>)
            -> Result<(), sp_mmr_primitives::Error>
        {
            pub type MmrLeaf = <<Runtime as pallet_mmr::Config>::LeafData as sp_mmr_primitives::LeafDataProvider>::LeafData;
            let leaves = leaves.into_iter().map(|leaf|
                leaf.into_opaque_leaf()
                .try_decode()
                .ok_or(sp_mmr_primitives::Error::Verify)).collect::<Result<Vec<MmrLeaf>, sp_mmr_primitives::Error>>()?;
            Mmr::verify_leaves(leaves, proof)
        }

        fn verify_proof_stateless(
            root: Hash,
            leaves: Vec<sp_mmr_primitives::EncodableOpaqueLeaf>,
            proof: sp_mmr_primitives::Proof<Hash>
        ) -> Result<(), sp_mmr_primitives::Error> {
            let nodes = leaves.into_iter().map(|leaf|sp_mmr_primitives::DataOrHash::Data(leaf.into_opaque_leaf())).collect();
            pallet_mmr::verify_leaves_proof::<<Runtime as pallet_mmr::Config>::Hashing, _>(root, nodes, proof)
        }
    }

    impl beefy_light_client_runtime_api::BeefyLightClientAPI<Block, beefy_light_client::BitField> for Runtime {
        fn get_random_bitfield(network_id: SubNetworkId, prior: beefy_light_client::BitField, num_of_validators: u32) -> beefy_light_client::BitField {
            let len = prior.len();
            BeefyLightClient::create_random_bit_field(network_id, prior, num_of_validators).unwrap_or(beefy_light_client::BitField::with_capacity(len))
        }
    }

    impl leaf_provider_runtime_api::LeafProviderAPI<Block> for Runtime {
        fn latest_digest() -> Option<bridge_types::types::AuxiliaryDigest> {
                LeafProvider::latest_digest().map(|logs| bridge_types::types::AuxiliaryDigest{ logs })
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(account: AccountId) -> Index {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
        fn query_info(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
        fn query_weight_to_fee(weight: Weight) -> Balance {
            TransactionPayment::weight_to_fee(weight)
        }
        fn query_length_to_fee(length: u32) -> Balance {
            TransactionPayment::length_to_fee(length)
        }
    }

    impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
        fn collect_collation_info(header: &<Block as BlockT>::Header) -> cumulus_primitives_core::CollationInfo {
            ParachainSystem::collect_collation_info(header)
        }
    }

    #[cfg(feature = "try-runtime")]
    impl frame_try_runtime::TryRuntime<Block> for Runtime {
        fn on_runtime_upgrade() -> (Weight, Weight) {
            log::info!("try-runtime::on_runtime_upgrade parachain-template.");
            let weight = Executive::try_runtime_upgrade().unwrap();
            (weight, RuntimeBlockWeights::get().max_block)
        }

        fn execute_block_no_check(block: Block) -> Weight {
            Executive::execute_block_no_check(block)
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn benchmark_metadata(extra: bool) -> (
            Vec<frame_benchmarking::BenchmarkList>,
            Vec<frame_support::traits::StorageInfo>,
        ) {
            use frame_benchmarking::{list_benchmark, Benchmarking, BenchmarkList};
            use frame_support::traits::StorageInfoTrait;
            // use frame_system_benchmarking::Pallet as SystemBench;
            // use cumulus_pallet_session_benchmarking::Pallet as SessionBench;

            let mut list = Vec::<BenchmarkList>::new();
            list_benchmark!(list, extra, xcm_app, XCMApp);
            list_benchmark!(list, extra, pallet_xcm, PolkadotXcm);

            let storage_info = AllPalletsWithSystem::storage_info();
            (list, storage_info)
        }

        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{Benchmarking, BenchmarkBatch, TrackedStorageKey, add_benchmark};

            // use frame_system_benchmarking::Pallet as SystemBench;
            impl frame_system_benchmarking::Config for Runtime {}

            // use cumulus_pallet_session_benchmarking::Pallet as SessionBench;
            impl cumulus_pallet_session_benchmarking::Config for Runtime {}

            let whitelist: Vec<TrackedStorageKey> = vec![
                // Block Number
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
                // Total Issuance
                hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
                // Execution Phase
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
                // Event Count
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
                // System Events
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
            ];

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);
            add_benchmark!(params, batches, xcm_app, XCMApp);
            add_benchmark!(params, batches, pallet_xcm, PolkadotXcm);

            if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
            Ok(batches)
        }
    }
}

struct CheckInherents;

impl cumulus_pallet_parachain_system::CheckInherents<Block> for CheckInherents {
    fn check_inherents(
        block: &Block,
        relay_state_proof: &cumulus_pallet_parachain_system::RelayChainStateProof,
    ) -> sp_inherents::CheckInherentsResult {
        let relay_chain_slot = relay_state_proof
            .read_slot()
            .expect("Could not read the relay chain slot from the proof");

        let inherent_data =
            cumulus_primitives_timestamp::InherentDataProvider::from_relay_chain_slot_and_duration(
                relay_chain_slot,
                sp_std::time::Duration::from_secs(6),
            )
            .create_inherent_data()
            .expect("Could not create the timestamp inherent data");

        inherent_data.check_extrinsics(block)
    }
}

cumulus_pallet_parachain_system::register_validate_block! {
    Runtime = Runtime,
    BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
    CheckInherents = CheckInherents,
}
