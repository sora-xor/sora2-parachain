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


pub use pallet::*;
// use xcm::latest::prelude::*;


// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use common::primitives::AssetId;
	use xcm::v1::{MultiLocation, MultiAsset};
	use xcm::opaque::latest::{Fungibility::Fungible, AssetId::Concrete};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
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

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		// /// Error names should be descriptive.
		// NoneValue,
		// /// Errors should have helpful documentation associated with them.
		// StorageOverflow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn register_pair(origin: OriginFor<T>, asset_id: AssetId, multilocation: MultiLocation) -> DispatchResultWithPostInfo {
			let _ = ensure_root(origin);
			AssetIdToMultilocation::<T>::insert(asset_id, multilocation.clone());
			MultilocationToAssetId::<T>::insert(multilocation, asset_id);
			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn change_pair(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn delete_pair(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			Ok(().into())
		}
	}

	impl<T: Config> sp_runtime::traits::Convert<AssetId, Option<MultiLocation>> for Pallet<T> {
		fn convert(id: AssetId) -> Option<MultiLocation> {
			Pallet::<T>::get_multilocation_from_asset_id(id)
		}
	}

	impl<T: Config> sp_runtime::traits::Convert<MultiLocation, Option<AssetId>> for Pallet<T> {
		fn convert(multilocation: MultiLocation) -> Option<AssetId> {
			Pallet::<T>::get_asset_id_from_multilocation(multilocation)
		}
	}

	impl<T: Config> sp_runtime::traits::Convert<MultiAsset, Option<AssetId>> for Pallet<T> {
		fn convert(a: MultiAsset) -> Option<AssetId> {
			if let MultiAsset { fun: Fungible(_), id: Concrete(id) } = a {
				Self::convert(id)
			} else {
				Option::None
			}
		}
	}
}