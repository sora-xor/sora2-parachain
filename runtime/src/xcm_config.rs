use super::{
	AccountId, Balances, Call, Event, Origin, ParachainInfo, ParachainSystem, PolkadotXcm, Runtime,
	WeightToFee, XcmpQueue,
};
use crate::sp_api_hidden_includes_construct_runtime::hidden_include::traits::Get;
use core::marker::PhantomData;
use frame_support::{
	log, match_types, parameter_types,
	traits::{Everything, Nothing},
	weights::Weight,
};
use orml_traits::{location::AbsoluteReserveProvider, parameter_type_with_key, MultiCurrency};
use orml_xcm_support::{
	DepositToAlternative, IsNativeConcrete, MultiCurrencyAdapter, MultiNativeAsset,
};
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use polkadot_runtime_common::impls::ToAuthor;
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom, CurrencyAdapter,
	EnsureXcmOrigin, FixedWeightBounds, IsConcrete, LocationInverter, NativeAsset, ParentIsPreset,
	RelayChainAsNative, SiblingParachainAsNative, SiblingParachainConvertsVia,
	SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit,
	UsingComponents,
};
use xcm_executor::{traits::ShouldExecute, XcmExecutor};
use crate::CurrencyId;
// use common::primitives::CurrencyId;

parameter_types! {
	pub const RelayLocation: MultiLocation = MultiLocation::parent();
	pub const RelayNetwork: NetworkId = NetworkId::Any;
	pub RelayChainOrigin: Origin = cumulus_pallet_xcm::Origin::Relay.into();
	pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
}

/// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
	// The parent (Relay-chain) origin converts to the parent `AccountId`.
	ParentIsPreset<AccountId>,
	// Sibling parachain origins convert to AccountId via the `ParaId::into`.
	SiblingParachainConvertsVia<Sibling, AccountId>,
	// Straight up local `AccountId32` origins just alias directly to `AccountId`.
	AccountId32Aliases<RelayNetwork, AccountId>,
);

/// Means for transacting assets on this chain.
// pub type LocalAssetTransactor = CurrencyAdapter<
// 	// Use this currency:
// 	Balances,
// 	// Use this currency when it is a fungible asset matching the given location or name:
// 	IsConcrete<RelayLocation>,
// 	// Do a simple punn to convert an AccountId32 MultiLocation into a native chain account ID:
// 	LocationToAccountId,
// 	// Our chain's account ID type (we can't get away without mentioning it explicitly):
// 	AccountId,
// 	// We don't track any teleports.
// 	(),
// >;

pub type LocalAssetTransactor = MultiCurrencyAdapter<
	crate::Tokens,
	(),
	IsNativeConcrete<CurrencyId, CurrencyIdConvert>,
	AccountId,
	LocationToAccountId,
	CurrencyId,
	// common::primitives::AssetId,
	CurrencyIdConvert,
	// DepositToAlternative<KaruraTreasuryAccount, Currencies, CurrencyId, AccountId, Balance>,
	(),
>;

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
	// Sovereign account converter; this attempts to derive an `AccountId` from the origin location
	// using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
	// foreign chains who want to have a local sovereign account on this chain which they control.
	SovereignSignedViaLocation<LocationToAccountId, Origin>,
	// Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
	// recognized.
	RelayChainAsNative<RelayChainOrigin, Origin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognized.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, Origin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `Origin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, Origin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<Origin>,
);

parameter_types! {
	// One XCM operation is 1_000_000_000 weight - almost certainly a conservative estimate.
	pub UnitWeightCost: Weight = 1_000_000_000;
	pub const MaxInstructions: u32 = 100;
}

match_types! {
	pub type ParentOrParentsExecutivePlurality: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(Plurality { id: BodyId::Executive, .. }) }
	};
}

//TODO: move DenyThenTry to polkadot's xcm module.
/// Deny executing the xcm message if it matches any of the Deny filter regardless of anything else.
/// If it passes the Deny, and matches one of the Allow cases then it is let through.
pub struct DenyThenTry<Deny, Allow>(PhantomData<Deny>, PhantomData<Allow>)
where
	Deny: ShouldExecute,
	Allow: ShouldExecute;

impl<Deny, Allow> ShouldExecute for DenyThenTry<Deny, Allow>
where
	Deny: ShouldExecute,
	Allow: ShouldExecute,
{
	fn should_execute<Call>(
		origin: &MultiLocation,
		message: &mut Xcm<Call>,
		max_weight: Weight,
		weight_credit: &mut Weight,
	) -> Result<(), ()> {
		Deny::should_execute(origin, message, max_weight, weight_credit)?;
		Allow::should_execute(origin, message, max_weight, weight_credit)
	}
}

// See issue #5233
pub struct DenyReserveTransferToRelayChain;
impl ShouldExecute for DenyReserveTransferToRelayChain {
	fn should_execute<Call>(
		origin: &MultiLocation,
		message: &mut Xcm<Call>,
		_max_weight: Weight,
		_weight_credit: &mut Weight,
	) -> Result<(), ()> {
		if message.0.iter().any(|inst| {
			matches!(
				inst,
				InitiateReserveWithdraw {
					reserve: MultiLocation { parents: 1, interior: Here },
					..
				} | DepositReserveAsset { dest: MultiLocation { parents: 1, interior: Here }, .. }
					| TransferReserveAsset {
						dest: MultiLocation { parents: 1, interior: Here },
						..
					}
			)
		}) {
			return Err(()); // Deny
		}

		// allow reserve transfers to arrive from relay chain
		if matches!(origin, MultiLocation { parents: 1, interior: Here })
			&& message.0.iter().any(|inst| matches!(inst, ReserveAssetDeposited { .. }))
		{
			log::warn!(
				target: "xcm::barriers",
				"Unexpected ReserveAssetDeposited from the relay chain",
			);
		}
		// Permit everything else
		Ok(())
	}
}

// pub type Barrier = DenyThenTry<
// 	DenyReserveTransferToRelayChain,
// 	(
// 		TakeWeightCredit,
// 		AllowTopLevelPaidExecutionFrom<Everything>,
// 		AllowUnpaidExecutionFrom<ParentOrParentsExecutivePlurality>,
// 		// ^^^ Parent and its exec plurality get free execution
// 	),
// >;

pub type Barrier = (TakeWeightCredit, AllowTopLevelPaidExecutionFrom<Everything>);

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type Call = Call;
	type XcmSender = XcmRouter;
	// How to withdraw and deposit an asset.
	type AssetTransactor = LocalAssetTransactor;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	// type IsReserve = NativeAsset;
	type IsReserve = MultiNativeAsset<AbsoluteReserveProvider>;
	// type IsTeleporter = (); // Teleporting is disabled.
	type IsTeleporter = NativeAsset; // Teleporting is disabled.
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = Barrier;
	// type Barrier = ();
	type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
	// type Trader =
	// 	UsingComponents<WeightToFee, RelayLocation, AccountId, Balances, ToAuthor<Runtime>>;
	type Trader = AllTokensAreCreatedEqualToWeight;
	type ResponseHandler = PolkadotXcm;
	type AssetTrap = PolkadotXcm;
	type AssetClaims = PolkadotXcm;
	type SubscriptionService = PolkadotXcm;
}

/// No local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation = SignedToAccountId32<Origin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, ()>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
);

impl pallet_xcm::Config for Runtime {
	type Event = Event;
	type SendXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
	// type XcmExecuteFilter = Nothing;
	type XcmExecuteFilter = Everything;
	// ^ Disable dispatchable execute on the XCM pallet.
	// Needs to be `Everything` for local testing.
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Everything;
	// type XcmReserveTransferFilter = Nothing;
	type XcmReserveTransferFilter = Everything;
	type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
	type LocationInverter = LocationInverter<Ancestry>;
	type Origin = Origin;
	type Call = Call;

	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	// ^ Override for AdvertisedXcmVersion default
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}

impl cumulus_pallet_xcm::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

parameter_types! {
	pub SelfLocation: MultiLocation = MultiLocation::new(1, X1(Parachain(ParachainInfo::get().into())));
}

parameter_types! {
	pub const BaseXcmWeight: Weight = 100_000_000; // TODO: recheck this
	pub const MaxAssetsForTransfer: usize = 2;
}

parameter_type_with_key! {
	pub ParachainMinFee: |location: MultiLocation| -> Option<u128> {
		#[allow(clippy::match_ref_pats)] // false positive
		match (location.parents, location.first_interior()) {
			(1, Some(_)) => Some(1_000_000),
			_ => None,
		}
	};
}

pub struct CurrencyIdConvert;
pub const XSTUSD_PREFIX: &[u8; 6] = b"XSTUSD";
// use crate::CurrencyId;
// impl sp_runtime::traits::Convert<common::primitives::AssetId, Option<MultiLocation>> for CurrencyIdConvert {
// 	// use parity_scale_codec::Encode;
// 	fn convert(id: common::primitives::AssetId) -> Option<MultiLocation> {
// 		// use primitives::TokenSymbol::*;
// 		// use CurrencyId::{Erc20, ForeignAsset, LiquidCrowdloan, StableAssetPoolToken, Token};
// 		// None
// 		// match id {
// 		// 	Token(DOT) => Some(MultiLocation::parent()),
// 		// 	Token(ACA) | Token(AUSD) | Token(LDOT) | Token(TAP) => {
// 		// 		Some(native_currency_location(ParachainInfo::get().into(), id.encode()))
// 		// 	}
// 		// 	Erc20(address) if !is_system_contract(address) => {
// 		// 		Some(native_currency_location(ParachainInfo::get().into(), id.encode()))
// 		// 	}
// 		// 	LiquidCrowdloan(_lease) => Some(native_currency_location(ParachainInfo::get().into(), id.encode())),
// 		// 	StableAssetPoolToken(_pool_id) => Some(native_currency_location(ParachainInfo::get().into(), id.encode())),
// 		// 	ForeignAsset(foreign_asset_id) => AssetIdMaps::<Runtime>::get_multi_location(foreign_asset_id),
// 		// 	_ => None,
// 		// }
// 		// match id {
// 		// 	// common::primitives::
// 		// 	_ => None,
// 		// }
// 		// Some(MultiLocation { parents: 1, interior:Here })
// 		Some(native_currency_location(ParachainInfo::get().into(), id))
// 	}
// }

impl sp_runtime::traits::Convert<CurrencyId, Option<MultiLocation>> for CurrencyIdConvert {
	// use parity_scale_codec::Encode;
	fn convert(id: CurrencyId) -> Option<MultiLocation> {
		match id {
			CurrencyId::XOR => Some(Parent.into()),
			CurrencyId::XSTUSD => Some(
				// (
				// 	Parent,
				// 	Parachain(2000),
				// 	GeneralKey(b"XSTUSD".to_vec().try_into().unwrap()),
				// )
				// 	.into(),
				MultiLocation::new(
					1,
					X2(Parachain(2000), GeneralKey(b"XSTUSD".to_vec().try_into().unwrap())),
				),
			),
		}
	}
}

impl sp_runtime::traits::Convert<MultiLocation, Option<CurrencyId>> for CurrencyIdConvert {
	// use parity_scale_codec::Encode;
	fn convert(l: MultiLocation) -> Option<CurrencyId> {
		let a: Vec<u8> = "XSTUSD".into();
		if l == MultiLocation::parent() {
			return Some(CurrencyId::XOR);
		}

		match l {
			MultiLocation { parents, interior } if parents == 1 => match interior {
				X2(Parachain(2000), GeneralKey(k)) if k == a => Some(CurrencyId::XSTUSD),
				_ => None,
			},
			MultiLocation { parents, interior } if parents == 0 => match interior {
				X1(GeneralKey(k)) if k == a => Some(CurrencyId::XSTUSD),
				_ => None,
			},
			_ => None,
		}
	}
}

impl sp_runtime::traits::Convert<MultiAsset, Option<CurrencyId>> for CurrencyIdConvert {
	// use parity_scale_codec::Encode;
	fn convert(a: MultiAsset) -> Option<CurrencyId> {
		if let MultiAsset { fun: Fungible(_), id: Concrete(id) } = a {
			Self::convert(id)
		} else {
			Option::None
		}
	}
}

pub struct AccountIdToMultiLocation;
impl sp_runtime::traits::Convert<AccountId, MultiLocation> for AccountIdToMultiLocation {
	fn convert(account: AccountId) -> MultiLocation {
		X1(AccountId32 { network: NetworkId::Any, id: account.into() }).into()
	}
}

impl orml_xtokens::Config for Runtime {
	type Event = Event;
	type Balance = crate::Balance;
	type CurrencyId = CurrencyId;
	type CurrencyIdConvert = CurrencyIdConvert;
	type AccountIdToMultiLocation = AccountIdToMultiLocation;
	type SelfLocation = SelfLocation;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type Weigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;
	type BaseXcmWeight = BaseXcmWeight;
	type LocationInverter = LocationInverter<Ancestry>;
	type MaxAssetsForTransfer = MaxAssetsForTransfer;
	type MinXcmFee = ParachainMinFee;
	type MultiLocationsFilter = Everything;
	type ReserveProvider = AbsoluteReserveProvider;
}

use frame_support::traits::{ConstU128, ConstU32, ConstU64};
use sp_std::vec::Vec;
// pub fn native_currency_location(para_id: u32, key: Vec<u8>) -> MultiLocation {
// 	MultiLocation::new(
// 		1,
// 		X2(
// 			Parachain(para_id),
// 			GeneralKey(frame_support::WeakBoundedVec::<u8, ConstU32<32>>::force_from(key, None).to_vec()),
// 		)
// 	)
// }

pub fn native_currency_location(para_id: u32, key: u64) -> MultiLocation {
	MultiLocation::new(1, X2(Parachain(para_id), GeneralKey(key.to_ne_bytes().to_vec())))
}






// TEMPORARYYYY!!!!!!
/// A trader who believes all tokens are created equal to "weight" of any chain,
/// which is not true, but good enough to mock the fee payment of XCM execution.
///
/// This mock will always trade `n` amount of weight to `n` amount of tokens.
pub struct AllTokensAreCreatedEqualToWeight(MultiLocation);
impl xcm_executor::traits::WeightTrader for AllTokensAreCreatedEqualToWeight {
	fn new() -> Self {
		Self(MultiLocation::parent())
	}

	fn buy_weight(&mut self, weight: Weight, payment: xcm_executor::Assets) -> Result<xcm_executor::Assets, XcmError> {
		let asset_id = payment
			.fungible
			.iter()
			.next()
			.expect("Payment must be something; qed")
			.0;
		let required = MultiAsset {
			id: asset_id.clone(),
			fun: Fungible(weight as u128),
		};

		if let MultiAsset {
			fun: _,
			id: Concrete(ref id),
		} = &required
		{
			self.0 = id.clone();
		}

		let unused = payment.checked_sub(required).map_err(|_| XcmError::TooExpensive)?;
		Ok(unused)
	}

	fn refund_weight(&mut self, weight: Weight) -> Option<MultiAsset> {
		if weight == 0 {
			None
		} else {
			Some((self.0.clone(), weight as u128).into())
		}
	}
}