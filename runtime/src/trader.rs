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

use xcm::{latest::Weight as XcmWeight, prelude::*};
use xcm_executor::{traits::WeightTrader, Assets};
use crate::XCMApp;

/// Does not take any fees but checks if there are enought assets to pass through the bridge
pub struct ParachainTrader;

impl WeightTrader for ParachainTrader {
    fn new() -> Self {
        log::trace!(target: "xcm::weight", "creating new WeightTrader instance");
        Self
    }

    fn buy_weight(&mut self, weight: XcmWeight, assets: Assets) -> Result<Assets, XcmError> {
        log::trace!(target: "xcm::weight", "buy_weight weight: {:?}, payment: {:?}", weight, assets);
        if assets.fungible.len() == 0 {
            return Err(XcmError::AssetNotFound);
        }

        for (asset_id, val) in &assets.fungible {
            let ml = match asset_id {
                Concrete(m) => m,
                _ => return Err(XcmError::AssetNotFound),
            };
            let Some(minimum_amount) = XCMApp::asset_minimum_amount(ml) else {
                return Err(XcmError::TooExpensive);
            };
            if *val < minimum_amount {
                return Err(XcmError::TooExpensive);
            }
        }

        Ok(assets)
    }
}
