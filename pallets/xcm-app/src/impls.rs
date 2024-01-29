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

use crate::*;
use frame_support::fail;
use sp_runtime::traits::Convert;

// IMPLS
impl<T: Config> MultiCurrency<T::AccountId> for Pallet<T> {
    type CurrencyId = AssetId;
    type Balance = T::Balance;

    fn minimum_balance(_currency_id: Self::CurrencyId) -> Self::Balance {
        log::trace!(
            target: "xcm::XCMApp",
            "minimum_balance",
        );
        Default::default()
    }

    fn total_issuance(_currency_id: Self::CurrencyId) -> Self::Balance {
        log::trace!(
            target: "xcm::XCMApp",
            "total_issuance",
        );
        Default::default()
    }

    fn total_balance(_currency_id: Self::CurrencyId, _who: &T::AccountId) -> Self::Balance {
        log::trace!(
            target: "xcm::XCMApp",
            "total_balance",
        );
        Default::default()
    }

    fn free_balance(_currency_id: Self::CurrencyId, _who: &T::AccountId) -> Self::Balance {
        log::trace!(
            target: "xcm::XCMApp",
            "free_balance",
        );
        Default::default()
    }

    fn ensure_can_withdraw(
        _currency_id: Self::CurrencyId,
        _who: &T::AccountId,
        _amount: Self::Balance,
    ) -> sp_runtime::DispatchResult {
        log::trace!(
            target: "xcm::XCMApp",
            "ensure_can_withdraw",
        );
        fail!(Error::<T>::MethodNotAvailible)
    }

    fn transfer(
        _currency_id: Self::CurrencyId,
        _from: &T::AccountId,
        _to: &T::AccountId,
        _amount: Self::Balance,
    ) -> sp_runtime::DispatchResult {
        log::trace!(
            target: "xcm::XCMApp",
            "transfer",
        );
        Ok(())
    }

    /// THIS
    fn deposit(
        currency_id: Self::CurrencyId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> sp_runtime::DispatchResult {
        log::trace!(
            target: "xcm::XCMApp",
            "deposit",
        );
        Pallet::<T>::add_to_channel(
            who.clone(),
            currency_id,
            T::BalanceConverter::convert(amount),
        )?;
        Ok(())
    }

    fn withdraw(
        _currency_id: Self::CurrencyId,
        _who: &T::AccountId,
        _amount: Self::Balance,
    ) -> sp_runtime::DispatchResult {
        log::trace!(
            target: "xcm::XCMApp",
            "withdraw",
        );
        Ok(())
    }

    fn can_slash(
        _currency_id: Self::CurrencyId,
        _who: &T::AccountId,
        _value: Self::Balance,
    ) -> bool {
        log::trace!(
            target: "xcm::XCMApp",
            "can_slash",
        );
        false
    }

    fn slash(
        _currency_id: Self::CurrencyId,
        _who: &T::AccountId,
        _amount: Self::Balance,
    ) -> Self::Balance {
        Default::default()
    }
}

// IMPLS for p_runtime::traits::Convert trait to allow this pallet be used as Converter in XCM localasset transactor:

impl<T: Config> sp_runtime::traits::Convert<AssetId, Option<MultiLocation>> for Pallet<T> {
    fn convert(id: AssetId) -> Option<MultiLocation> {
        let maybe_multilocation = Pallet::<T>::get_multilocation_from_asset_id(id);
        if maybe_multilocation.is_none() {
            Self::deposit_event(Event::<T>::AssetIdMappingError(id));
        }
        maybe_multilocation
    }
}

impl<T: Config> sp_runtime::traits::Convert<MultiLocation, Option<AssetId>> for Pallet<T> {
    fn convert(multilocation: MultiLocation) -> Option<AssetId> {
        let maybe_asset_id = Pallet::<T>::get_asset_id_from_multilocation(multilocation);
        if maybe_asset_id.is_none() {
            Self::deposit_event(Event::<T>::MultilocationMappingError(multilocation));
        }
        maybe_asset_id
    }
}

impl<T: Config> sp_runtime::traits::Convert<MultiAsset, Option<AssetId>> for Pallet<T> {
    fn convert(ma: MultiAsset) -> Option<AssetId> {
        if let MultiAsset { fun: Fungible(_), id: Concrete(ml) } = ma {
            Self::convert(ml)
        } else {
            Self::deposit_event(Event::<T>::MultiAssetMappingError(ma));
            Option::None
        }
    }
}
