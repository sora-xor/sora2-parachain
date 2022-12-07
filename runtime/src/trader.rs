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

use sp_std::prelude::*;
use xcm::{latest::Weight as XcmWeight, prelude::*};
use xcm_executor::{traits::WeightTrader, Assets};

pub struct ParachainTrader {
	pub weight: XcmWeight,
	multi_location: Option<MultiLocation>,
}

impl WeightTrader for ParachainTrader {
	fn new() -> Self {
		log::trace!(target: "xcm::weight", "creating new WeightTrader instance");
		Self { weight: 0, multi_location: None }
	}

	fn buy_weight(&mut self, weight: XcmWeight, payment: Assets) -> Result<Assets, XcmError> {
		log::trace!(target: "xcm::weight", "buy_weight weight: {:?}, payment: {:?}", weight, payment);
		let asset_id = payment
			.fungible
			.iter()
			.next()
			.map_or(Err(XcmError::TooExpensive), |v| Ok(v.0))?;

		let required = MultiAsset { id: asset_id.clone(), fun: Fungible(weight as u128) };

		if let MultiAsset { fun: _, id: Concrete(ref id) } = &required {
			self.multi_location = Some(id.clone());
		} else {
		}

		let unused = payment.checked_sub(required).map_err(|_| XcmError::TooExpensive)?;
		Ok(unused)
	}

	fn refund_weight(&mut self, weight: XcmWeight) -> Option<MultiAsset> {
		log::trace!(
			target: "xcm::weight", "refund_weight weight: {:?} ",
			weight
		);
		match &self.multi_location {
			None => None,
			Some(ml) =>
				if weight == 0 {
					None
				} else {
					Some((ml.clone(), weight as u128).into())
				},
		}
	}
}
