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

use frame_support::{traits::OnRuntimeUpgrade, weights::Weight};
use sp_core::ecdsa;
use sp_runtime::impl_opaque_keys;
use sp_std::vec::Vec;
use crate::*;

use crate::{AccountId, Aura, BeefyId, RuntimeBlockWeights, Session};

// pub type Migrations = SessionKeysMigration;
pub type Migrations = (
    pallet_balances::migration::MigrateManyToTrackInactive<Runtime, EmptyAccountList>,
    DummyMigration,
    SuperDummyMigration,
);

impl_opaque_keys! {
    pub struct SessionKeysOld {
        pub aura: Aura,
    }
}

/// Generates a `BeefyId` from the given `AccountId`. The resulting `BeefyId` is
/// a dummy value and this is a utility function meant to be used when migration
/// session keys.
pub fn dummy_beefy_id_from_account_id(a: AccountId) -> BeefyId {
    let mut id_raw = [0u8; 33];

    // NOTE: AccountId is 32 bytes, whereas BeefyId is 33 bytes.
    id_raw[1..].copy_from_slice(a.as_ref());
    id_raw[0..4].copy_from_slice(b"beef");

    ecdsa::Public(id_raw).into()
}

pub struct SessionKeysMigration;

impl OnRuntimeUpgrade for SessionKeysMigration {
    fn on_runtime_upgrade() -> Weight {
        Session::upgrade_keys::<SessionKeysOld, _>(|id, keys| crate::SessionKeys {
            aura: keys.aura,
            beefy: dummy_beefy_id_from_account_id(id),
        });
        RuntimeBlockWeights::get().max_block
    }
}

pub struct DummyMigration;

impl OnRuntimeUpgrade for DummyMigration {
    fn on_runtime_upgrade() -> Weight {
        // Session::upgrade_keys::<SessionKeysOld, _>(|id, keys| crate::SessionKeys {
        //     aura: keys.aura,
        //     beefy: dummy_beefy_id_from_account_id(id),
        // });
        // RuntimeBlockWeights::get().max_block
        // 0.into()
        log::warn!(target: "runtime::xcm", "=============================");
        log::warn!(target: "runtime::xcm", "=============================");
        log::warn!(target: "runtime::xcm", "=============================");
        log::warn!(target: "runtime::xcm", "UPGRADE");
        log::warn!(target: "runtime::xcm", "=============================");
        log::warn!(target: "runtime::xcm", "=============================");
        log::warn!(target: "runtime::xcm", "=============================");
        Weight::zero()
    }
}

pub struct SuperDummyMigration;

impl OnRuntimeUpgrade for SuperDummyMigration {
    fn on_runtime_upgrade() -> Weight {
        // Session::upgrade_keys::<SessionKeysOld, _>(|id, keys| crate::SessionKeys {
        //     aura: keys.aura,
        //     beefy: dummy_beefy_id_from_account_id(id),
        // });
        // RuntimeBlockWeights::get().max_block
        // 0.into()
        log::warn!(target: "runtime::xcm", "++++++++++++++++++++++++++++");
        log::warn!(target: "runtime::xcm", "++++++++++++++++++++++++++++");
        log::warn!(target: "runtime::xcm", "++++++++++++++++++++++++++++");
        log::warn!(target: "runtime::xcm", "SUPER UPGRAGE");
        log::warn!(target: "runtime::xcm", "++++++++++++++++++++++++++++");
        log::warn!(target: "runtime::xcm", "++++++++++++++++++++++++++++");
        log::warn!(target: "runtime::xcm", "++++++++++++++++++++++++++++");
        Weight::zero()
    }
}


pub struct EmptyAccountList;

impl sp_core::Get<Vec<AccountId>> for EmptyAccountList {
    fn get() -> Vec<AccountId> {
        Default::default()
    }
}