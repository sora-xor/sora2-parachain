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

// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

// pub mod weights;

use common::primitives::AssetId;
use frame_support::weights::Weight;
pub use pallet::*;
use xcm::opaque::latest::{AssetId::Concrete, Fungibility::Fungible};
use xcm::v1::{MultiAsset, MultiLocation};
use orml_traits::{
	arithmetic::{self, Signed},
	currency::TransferAll,
	BalanceStatus, GetByKey, Happened, LockIdentifier, MultiCurrency, MultiCurrencyExtended, MultiLockableCurrency,
	MultiReservableCurrency, NamedMultiReservableCurrency, OnDust,
};

// IMPLS 
impl<T: Config> MultiCurrency<T::AccountId> for Pallet<T> {
	type CurrencyId = T::CurrencyId;
	type Balance = T::Balance;

    fn minimum_balance(currency_id: Self::CurrencyId) -> Self::Balance {
        Default::default()
    }

    fn total_issuance(currency_id: Self::CurrencyId) -> Self::Balance {
        Default::default()
    }

    fn total_balance(currency_id: Self::CurrencyId, who: &T::AccountId) -> Self::Balance {
        Default::default()
    }

    fn free_balance(currency_id: Self::CurrencyId, who: &T::AccountId) -> Self::Balance {
        Default::default()
    }

    fn ensure_can_withdraw(currency_id: Self::CurrencyId, who: &T::AccountId, amount: Self::Balance) -> sp_runtime::DispatchResult {
        todo!()
    }

    fn transfer(
		currency_id: Self::CurrencyId,
		from: &T::AccountId,
		to: &T::AccountId,
		amount: Self::Balance,
	) -> sp_runtime::DispatchResult {
		log::trace!(
			target: "xcm::transactor",
			"transfer",
		);
		Ok(())
    }

    fn deposit(currency_id: Self::CurrencyId, who: &T::AccountId, amount: Self::Balance) -> sp_runtime::DispatchResult {
		log::trace!(
			target: "xcm::transactor",
			"deposit",
		);
		Ok(())
    }

    fn withdraw(currency_id: Self::CurrencyId, who: &T::AccountId, amount: Self::Balance) -> sp_runtime::DispatchResult {
		log::trace!(
			target: "xcm::transactor",
			"withdraw",
		);
		Ok(())
    }

    fn can_slash(currency_id: Self::CurrencyId, who: &T::AccountId, value: Self::Balance) -> bool {
		log::trace!(
			target: "xcm::transactor",
			"can_slash",
		);
		false
    }

    fn slash(currency_id: Self::CurrencyId, who: &T::AccountId, amount: Self::Balance) -> Self::Balance {
        todo!()
    }
}

pub trait WeightInfo {
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResultWithPostInfo, fail, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		// type WeightInfo: WeightInfo;
		/// The balance type
		type Balance: Parameter
		+ Member
		+ sp_runtime::traits::AtLeast32BitUnsigned
		+ Default
		+ Copy
		+ MaybeSerializeDeserialize
		+ MaxEncodedLen;

		/// The currency ID type
		type CurrencyId: Parameter + Member + Copy + MaybeSerializeDeserialize + Ord + TypeInfo + MaxEncodedLen;
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
	}

	#[pallet::error]
	pub enum Error<T> {
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	impl<T: Config> Pallet<T> {

	}
}
