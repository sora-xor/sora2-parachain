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

#![allow(unused_parens)]
#![allow(unused_imports)]

use core::marker::PhantomData;
use frame_support::{traits::Get, weights::Weight};
use parachain_common::primitives::EXTRINSIC_FIXED_WEIGHT;

/// Weight functions for `XCMApp`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfo for SubstrateWeight<T> {
	/// Storage: XCMApp AssetIdToMultilocation (r:1 w:1)
	/// Proof Skipped: XCMApp AssetIdToMultilocation (max_values: None, max_size: None, mode: Measured)
	/// Storage: XCMApp MultilocationToAssetId (r:1 w:1)
	/// Proof Skipped: XCMApp MultilocationToAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel MessageQueues (r:1 w:1)
	/// Proof Skipped: SubstrateBridgeOutboundChannel MessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel ChannelNonces (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeOutboundChannel ChannelNonces (max_values: None, max_size: None, mode: Measured)
	/// Storage: XCMApp AssetMinimumAmount (r:0 w:1)
	/// Proof Skipped: XCMApp AssetMinimumAmount (max_values: None, max_size: None, mode: Measured)
	fn register_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37`
		//  Estimated: `10085`
		// Minimum execution time: 47_000 nanoseconds.
		Weight::from_parts(55_000_000, 10085)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: XCMApp AssetIdToMultilocation (r:1 w:0)
	/// Proof Skipped: XCMApp AssetIdToMultilocation (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: XCMApp MultilocationToAssetId (r:1 w:0)
	/// Proof Skipped: XCMApp MultilocationToAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel MessageQueues (r:1 w:1)
	/// Proof Skipped: SubstrateBridgeOutboundChannel MessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel ChannelNonces (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeOutboundChannel ChannelNonces (max_values: None, max_size: None, mode: Measured)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `694`
		//  Estimated: `15553`
		// Minimum execution time: 79_000 nanoseconds.
		Weight::from_parts(92_000_000, 15553)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: XCMApp BridgeAssetTrap (r:1 w:1)
	/// Proof Skipped: XCMApp BridgeAssetTrap (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel MessageQueues (r:1 w:1)
	/// Proof Skipped: SubstrateBridgeOutboundChannel MessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel ChannelNonces (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeOutboundChannel ChannelNonces (max_values: None, max_size: None, mode: Measured)
	fn try_claim_bridge_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `276`
		//  Estimated: `8253`
		// Minimum execution time: 20_000 nanoseconds.
		Weight::from_parts(21_000_000, 8253)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: XCMApp AssetIdToMultilocation (r:1 w:0)
	/// Proof Skipped: XCMApp AssetIdToMultilocation (max_values: None, max_size: None, mode: Measured)
	/// Storage: XCMApp AssetMinimumAmount (r:0 w:1)
	/// Proof Skipped: XCMApp AssetMinimumAmount (max_values: None, max_size: None, mode: Measured)
	fn set_asset_minimum_amount() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `438`
		//  Estimated: `3351`
		// Minimum execution time: 10_000 nanoseconds.
		Weight::from_parts(12_000_000, 3351)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}

	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
    fn sudo_send_xcm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `1002`
		// Minimum execution time: 13_000 nanoseconds.
		Weight::from_parts(14_000_000, 1002)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
    }
}

impl crate::WeightInfo for () {
    fn transfer() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }

    fn register_asset() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }

    fn try_claim_bridge_asset() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }

    fn set_asset_minimum_amount() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }

    fn sudo_send_xcm() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
}
