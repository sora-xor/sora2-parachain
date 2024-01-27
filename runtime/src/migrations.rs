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

#[cfg(feature = "kusama")]
use crate::*;
#[cfg(feature = "kusama")]
use frame_support::{traits::OnRuntimeUpgrade, weights::Weight};

#[cfg(feature = "kusama")]
pub type Migrations = (RemoveSudoKey,);

#[cfg(feature = "rococo")]
pub type Migrations = ();

#[cfg(feature = "polkadot")]
pub type Migrations =
    (pallet_balances::migration::MigrateManyToTrackInactive<crate::Runtime, EmptyAccountList>,);

#[cfg(feature = "polkadot")]
pub struct EmptyAccountList;

#[cfg(feature = "polkadot")]
impl sp_core::Get<crate::Vec<crate::AccountId>> for EmptyAccountList {
    fn get() -> crate::Vec<crate::AccountId> {
        Default::default()
    }
}

#[cfg(feature = "kusama")]
pub struct RemoveSudoKey;

#[cfg(feature = "kusama")]
impl OnRuntimeUpgrade for RemoveSudoKey {
    fn on_runtime_upgrade() -> Weight {
        if let Some(key) =
            frame_support::storage::migration::take_storage_value::<AccountId>(b"Sudo", b"Key", &[])
        {
            frame_support::log::error!("Sudo key removed: {:?}", key);
        } else {
            frame_support::log::error!("Sudo key not found in storage");
        }
        RuntimeBlockWeights::get().max_block
    }
}
