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

use core::marker::PhantomData;

use frame_support::{
    dispatch::DispatchClass,
    weights::{constants::BlockExecutionWeight, Weight},
};

pub struct CollectiveWeightInfo<T>(PhantomData<T>);

pub struct PreimageWeightInfo;

const MAX_PREIMAGE_BYTES: u32 = 5 * 1024 * 1024;

impl pallet_preimage::WeightInfo for PreimageWeightInfo {
    fn note_preimage(bytes: u32) -> Weight {
        let max_weight: Weight = crate::RuntimeBlockWeights::get()
            .get(DispatchClass::Normal)
            .max_extrinsic
            .expect("Democracy pallet must have max extrinsic weight");
        if bytes > MAX_PREIMAGE_BYTES {
            return max_weight.saturating_add(Weight::from_parts(1, 0))
        }
        let weight = <() as pallet_preimage::WeightInfo>::note_preimage(bytes);
        let max_dispatch_weight: Weight = max_weight.saturating_sub(BlockExecutionWeight::get());
        // We want to keep it as high as possible, but can't risk having it reject,
        // so we always the base block execution weight as a max
        max_dispatch_weight.min(weight)
    }

    fn note_requested_preimage(s: u32) -> Weight {
        <() as pallet_preimage::WeightInfo>::note_requested_preimage(s)
    }

    fn note_no_deposit_preimage(s: u32) -> Weight {
        <() as pallet_preimage::WeightInfo>::note_no_deposit_preimage(s)
    }

    fn unnote_preimage() -> Weight {
        <() as pallet_preimage::WeightInfo>::unnote_preimage()
    }

    fn unnote_no_deposit_preimage() -> Weight {
        <() as pallet_preimage::WeightInfo>::unnote_no_deposit_preimage()
    }

    fn request_preimage() -> Weight {
        <() as pallet_preimage::WeightInfo>::request_preimage()
    }

    fn request_no_deposit_preimage() -> Weight {
        <() as pallet_preimage::WeightInfo>::request_no_deposit_preimage()
    }

    fn request_unnoted_preimage() -> Weight {
        <() as pallet_preimage::WeightInfo>::request_unnoted_preimage()
    }

    fn request_requested_preimage() -> Weight {
        <() as pallet_preimage::WeightInfo>::request_requested_preimage()
    }

    fn unrequest_preimage() -> Weight {
        <() as pallet_preimage::WeightInfo>::unrequest_preimage()
    }

    fn unrequest_unnoted_preimage() -> Weight {
        <() as pallet_preimage::WeightInfo>::unrequest_unnoted_preimage()
    }

    fn unrequest_multi_referenced_preimage() -> Weight {
        <() as pallet_preimage::WeightInfo>::unrequest_multi_referenced_preimage()
    }
}

impl<T: frame_system::Config> pallet_collective::WeightInfo for CollectiveWeightInfo<T> {
    fn set_members(m: u32, n: u32, p: u32) -> Weight {
        <() as pallet_collective::WeightInfo>::set_members(m, n, p)
    }
    fn execute(b: u32, m: u32) -> Weight {
        <() as pallet_collective::WeightInfo>::execute(b, m)
    }
    fn propose_execute(b: u32, m: u32) -> Weight {
        <() as pallet_collective::WeightInfo>::propose_execute(b, m)
    }
    fn propose_proposed(b: u32, m: u32, p: u32) -> Weight {
        <() as pallet_collective::WeightInfo>::propose_proposed(b, m, p)
    }
    fn vote(m: u32) -> Weight {
        <() as pallet_collective::WeightInfo>::vote(m)
    }
    fn close_early_disapproved(m: u32, p: u32) -> Weight {
        <() as pallet_collective::WeightInfo>::close_early_disapproved(m, p)
    }
    fn close_early_approved(bytes: u32, m: u32, p: u32) -> Weight {
        let max_weight: Weight = crate::RuntimeBlockWeights::get()
            .get(DispatchClass::Normal)
            .max_extrinsic
            .expect("Collective pallet must have max extrinsic weight");
        if bytes > MAX_PREIMAGE_BYTES {
            return max_weight.saturating_add(Weight::from_parts(1, 0))
        }
        let weight = <() as pallet_collective::WeightInfo>::close_early_approved(bytes, m, p);
        let max_dispatch_weight: Weight = max_weight.saturating_sub(BlockExecutionWeight::get());
        // We want to keep it as high as possible, but can't risk having it reject,
        // so we always the base block execution weight as a max
        max_dispatch_weight.min(weight)
    }
    fn close_disapproved(m: u32, p: u32) -> Weight {
        <() as pallet_collective::WeightInfo>::close_disapproved(m, p)
    }
    fn close_approved(bytes: u32, m: u32, p: u32) -> Weight {
        let max_weight: Weight = crate::RuntimeBlockWeights::get()
            .get(DispatchClass::Normal)
            .max_extrinsic
            .expect("Collective pallet must have max extrinsic weight");
        if bytes > MAX_PREIMAGE_BYTES {
            return max_weight.saturating_add(Weight::from_parts(1, 0))
        }
        let weight = <() as pallet_collective::WeightInfo>::close_approved(bytes, m, p);
        let max_dispatch_weight: Weight = max_weight.saturating_sub(BlockExecutionWeight::get());
        // We want to keep it as high as possible, but can't risk having it reject,
        // so we always the base block execution weight as a max
        max_dispatch_weight.min(weight)
    }
    fn disapprove_proposal(p: u32) -> Weight {
        <() as pallet_collective::WeightInfo>::disapprove_proposal(p)
    }
}
