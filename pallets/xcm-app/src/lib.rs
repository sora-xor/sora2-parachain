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

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

mod impls;

pub mod weights;

pub use pallet::*;

use bridge_types::substrate::XCMAppMessage;
use frame_support::weights::Weight;
use orml_traits::MultiCurrency;
use parachain_common::primitives::AssetId;
use xcm::opaque::latest::{AssetId::Concrete, Fungibility::Fungible};
use xcm::v1::{MultiAsset, MultiLocation};

pub type ParachainAssetId = xcm::VersionedMultiAsset;

pub trait WeightInfo {
	fn register_mapping() -> Weight;

	fn change_asset_mapping() -> Weight;

	fn change_multilocation_mapping() -> Weight;

	fn delete_mapping() -> Weight;
}

impl<T: Config> From<XCMAppMessage<T::AccountId, AssetId, T::Balance>> for Call<T> {
	fn from(value: XCMAppMessage<T::AccountId, AssetId, T::Balance>) -> Self {
		match value {
			XCMAppMessage::Transfer { sender, recipient, amount, asset_id } => {
				Call::transfer { sender, recipient, amount, asset_id }
			},
			XCMAppMessage::RegisterAsset { asset_id, sidechain_asset } => {
				Call::register_asset { asset_id, multiasset: sidechain_asset }
			},
		}
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use bridge_types::H256;
	use bridge_types::{
		substrate::{SubstrateBridgeMessageEncode, XCMAppMessage},
		traits::OutboundChannel,
		SubNetworkId,
	};
	use frame_support::{dispatch::DispatchResultWithPostInfo, fail, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use parachain_common::primitives::AssetId;
	use sp_runtime::traits::Convert;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;

		/// The balance type
		type Balance: Parameter
			+ Member
			+ sp_runtime::traits::AtLeast32BitUnsigned
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen;

		type CallOrigin: EnsureOrigin<
			Self::RuntimeOrigin,
			Success = bridge_types::types::CallOriginOutput<SubNetworkId, H256, ()>,
		>;

		type OutboundChannel: OutboundChannel<SubNetworkId, Self::AccountId, ()>;

		type AccountIdToMultiLocation: Convert<Self::AccountId, MultiLocation>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn get_multilocation_from_asset_id)]
	pub type AssetIdToMultilocation<T: Config> =
		StorageMap<_, Blake2_256, AssetId, MultiLocation, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_asset_id_from_multilocation)]
	pub type MultilocationToAssetId<T: Config> =
		StorageMap<_, Blake2_256, MultiLocation, AssetId, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Adding mapping has been performed
		/// [Sora AssetId, XCM Multilocation]
		MappingCreated(AssetId, MultiLocation),
		/// Asset mapping change has been performed
		/// [Sora AssetId, XCM Multilocation]
		AssetMappingChanged(AssetId, MultiLocation),
		/// Multilocation mapping change has been performed
		/// [Sora AssetId, XCM Multilocation]
		MultilocationtMappingChanged(AssetId, MultiLocation),
		/// Mapping delete has been performed
		/// [Sora AssetId, XCM Multilocation]
		MappingDeleted(AssetId, MultiLocation),
		/// Asset Added to channel
		/// [Currency Id, amount]
		AssetAddedToChannel(AssetId, T::Balance),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Given AssetId or/and Multilocation is/are already used in mapping
		MappingAlreadyExists,
		/// No mapping for AssetId and Multilocation exists
		MappingNotExist,
		/// Method not availible
		MethodNotAvailible,
		/// Wrong XCM version
		WrongXCMVersion,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(<T as Config>::WeightInfo::register_mapping())]
		#[frame_support::transactional]
		pub fn transfer(
			origin: OriginFor<T>,
			asset_id: AssetId,
			sender: T::AccountId,
			recipient: xcm::VersionedMultiLocation,
			amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let res = T::CallOrigin::ensure_origin(origin)?;
			frame_support::log::info!(
				"Call transfer with params: {:?} by {:?}",
				(asset_id, sender, recipient, amount),
				res
			);
			Ok(().into())
		}

		#[pallet::weight(<T as Config>::WeightInfo::register_mapping())]
		#[frame_support::transactional]
		pub fn register_asset(
			origin: OriginFor<T>,
			asset_id: AssetId,
			multiasset: xcm::VersionedMultiAsset,
		) -> DispatchResultWithPostInfo {
			let res = T::CallOrigin::ensure_origin(origin)?;
			frame_support::log::info!(
				"Call register_asset with params: {:?} by {:?}",
				(asset_id, multiasset.clone()),
				res
			);
			let multiasset = match multiasset {
				xcm::VersionedMultiAsset::V1(multiasset) => multiasset,
				_ => fail!(Error::<T>::WrongXCMVersion),
			};
			let multilocation = match multiasset.id {
				xcm::v2::AssetId::Concrete(location) => location,
				xcm::v2::AssetId::Abstract(_) => fail!(Error::<T>::WrongXCMVersion),
			};
			ensure!(
				AssetIdToMultilocation::<T>::get(asset_id).is_none()
					|| MultilocationToAssetId::<T>::get(multilocation.clone()).is_none(),
				Error::<T>::MappingAlreadyExists
			);
			AssetIdToMultilocation::<T>::insert(asset_id, multilocation.clone());
			MultilocationToAssetId::<T>::insert(multilocation.clone(), asset_id);
			Self::deposit_event(Event::<T>::MappingCreated(asset_id, multilocation));
			Ok(().into())
		}

		/// Perform registration for mapping of an AssetId <-> Multilocation
		///
		/// - `origin`: the root account on whose behalf the transaction is being executed,
		/// - `asset_id`: asset id in Sora Network,
		/// - `multilocation`: XCM multilocation of an asset,
		#[pallet::weight(<T as Config>::WeightInfo::register_mapping())]
		#[frame_support::transactional]
		pub fn register_mapping(
			origin: OriginFor<T>,
			asset_id: AssetId,
			multilocation: MultiLocation,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_root(origin)?;
			ensure!(
				AssetIdToMultilocation::<T>::get(asset_id).is_none()
					|| MultilocationToAssetId::<T>::get(multilocation.clone()).is_none(),
				Error::<T>::MappingAlreadyExists
			);
			AssetIdToMultilocation::<T>::insert(asset_id, multilocation.clone());
			MultilocationToAssetId::<T>::insert(multilocation.clone(), asset_id);
			Self::deposit_event(Event::<T>::MappingCreated(asset_id, multilocation));
			Ok(().into())
		}

		/// Perform change of mapping of an AssetId -> Multilocation
		///
		/// - `origin`: the root account on whose behalf the transaction is being executed,
		/// - `asset_id`: asset id in Sora Network,
		/// - `new_multilocation`: new XCM multilocation of an asset,
		#[pallet::weight(<T as Config>::WeightInfo::change_asset_mapping())]
		#[frame_support::transactional]
		pub fn change_asset_mapping(
			origin: OriginFor<T>,
			asset_id: AssetId,
			new_multilocation: MultiLocation,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_root(origin)?;
			AssetIdToMultilocation::<T>::try_mutate(asset_id, |ml_opt| -> DispatchResult {
				match ml_opt {
					None => fail!(Error::<T>::MappingNotExist),
					Some(ml) => {
						// ensure that new_multilocation mapping does not exist
						ensure!(
							MultilocationToAssetId::<T>::get(new_multilocation.clone()).is_none(),
							Error::<T>::MappingAlreadyExists
						);
						MultilocationToAssetId::<T>::insert(new_multilocation.clone(), asset_id);

						// remove old multilocation
						MultilocationToAssetId::<T>::remove(ml.clone());

						*ml = new_multilocation.clone();
					},
				}
				Ok(())
			})?;
			Self::deposit_event(Event::<T>::AssetMappingChanged(asset_id, new_multilocation));
			Ok(().into())
		}

		/// Perform change of mapping of an Multilocation -> AssetId
		///
		/// - `origin`: the root account on whose behalf the transaction is being executed,
		/// - `multilocation`: XCM multilocation of an asset,
		/// - `new_asset_id`: new asset id in Sora Network,
		#[pallet::weight(<T as Config>::WeightInfo::change_multilocation_mapping())]
		#[frame_support::transactional]
		pub fn change_multilocation_mapping(
			origin: OriginFor<T>,
			multilocation: MultiLocation,
			new_asset_id: AssetId,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_root(origin)?;
			MultilocationToAssetId::<T>::try_mutate(
				multilocation.clone(),
				|asset_opt| -> DispatchResult {
					match asset_opt {
						None => fail!(Error::<T>::MappingNotExist),
						Some(asset_id) => {
							// ensure that new_assetid mapping does not exist
							ensure!(
								AssetIdToMultilocation::<T>::get(new_asset_id.clone()).is_none(),
								Error::<T>::MappingAlreadyExists
							);

							AssetIdToMultilocation::<T>::insert(
								new_asset_id,
								multilocation.clone(),
							);

							// remove old assetid
							AssetIdToMultilocation::<T>::remove(asset_id.clone());

							*asset_id = new_asset_id;
						},
					};
					Ok(())
				},
			)?;
			Self::deposit_event(Event::<T>::MultilocationtMappingChanged(
				new_asset_id,
				multilocation,
			));
			Ok(().into())
		}

		/// Perform delete of mapping of an AssetId -> Multilocation
		///
		/// - `origin`: the root account on whose behalf the transaction is being executed,
		/// - `asset_id`: asset id in Sora Network,
		#[pallet::weight(<T as Config>::WeightInfo::delete_mapping())]
		#[frame_support::transactional]
		pub fn delete_mapping(
			origin: OriginFor<T>,
			asset_id: AssetId,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_root(origin)?;
			match AssetIdToMultilocation::<T>::get(asset_id) {
				None => fail!(Error::<T>::MappingNotExist),
				Some(multilocation) => {
					AssetIdToMultilocation::<T>::remove(asset_id);
					MultilocationToAssetId::<T>::remove(multilocation.clone());
					Self::deposit_event(Event::<T>::MappingDeleted(asset_id, multilocation));
				},
			};
			Ok(().into())
		}

		#[pallet::weight(<T as Config>::WeightInfo::delete_mapping())]
		#[frame_support::transactional]
		pub fn fake_transfer(
			origin: OriginFor<T>,
			asset_id: AssetId,
			amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let account_id = ensure_signed(origin)?;
			Self::add_to_channel(account_id, asset_id, amount)?;
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn add_to_channel(
			account_id: T::AccountId,
			asset_id: AssetId,
			amount: T::Balance,
		) -> sp_runtime::DispatchResult {
			let raw_origin = Some(account_id.clone()).into();
			let multilocation =
				<T as Config>::AccountIdToMultiLocation::convert(account_id.clone());
			let xcm_mes = XCMAppMessage::Transfer {
				asset_id,
				sender: account_id.clone(),
				recipient: xcm::VersionedMultiLocation::V1(multilocation),
				amount,
			}
			.prepare_message();
			<T as Config>::OutboundChannel::submit(
				SubNetworkId::Mainnet,
				&raw_origin,
				&xcm_mes,
				(),
			)?;
			Self::deposit_event(Event::<T>::AssetAddedToChannel(asset_id, amount));
			Ok(())
		}
	}
}
