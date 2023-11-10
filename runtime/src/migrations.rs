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
use frame_support::{
    traits::{Currency, ExistenceRequirement, LockableCurrency, OnRuntimeUpgrade, WithdrawReasons},
    weights::Weight,
};
use sp_std::collections::btree_set::BTreeSet;

pub type Migrations = (RemoveMintedAccountsBalance,);

pub struct RemoveMintedAccountsBalance;

impl OnRuntimeUpgrade for RemoveMintedAccountsBalance {
    fn on_runtime_upgrade() -> Weight {
        let mut accounts_to_skip =
            pallet_collator_selection::Invulnerables::<Runtime>::get().to_vec();
        if let Some(sudo_key) = Sudo::key() {
            // Fees was updated, so we need to update sudo account balance
            // to be able to resolve possible issues after runtime upgrade.
            let _ = Balances::deposit_creating(&sudo_key, UNIT * 1000);
            accounts_to_skip.push(sudo_key);
        }
        let accounts_to_skip = BTreeSet::from_iter(accounts_to_skip.into_iter());
        for (account, info) in frame_system::Account::<Runtime>::iter() {
            if accounts_to_skip.contains(&account) {
                continue
            }
            let locks = Balances::locks(&account);
            for lock in locks {
                Balances::remove_lock(lock.id, &account);
            }
            if let Err(err) = Balances::withdraw(
                &account,
                info.data.free - EXISTENTIAL_DEPOSIT,
                WithdrawReasons::all(),
                ExistenceRequirement::KeepAlive,
            ) {
                frame_support::log::error!(
                    "Failed to withdraw funds from account {:?}: {:?}",
                    account,
                    err
                );
            }
        }
        RuntimeBlockWeights::get().max_block
    }
}
