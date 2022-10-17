use frame_support::weights::Weight;
use sp_std::prelude::*;
use xcm::latest::prelude::*;
use xcm_executor::{traits::WeightTrader, Assets};

pub struct ParachainTrader {
	pub weight: Weight,
	multi_location: Option<MultiLocation>,
}

impl WeightTrader for ParachainTrader {
	fn new() -> Self {
		log::trace!(target: "xcm::weight", "creating new WeightTrader instance");
		Self { weight: 0, multi_location: None }
	}

	fn buy_weight(&mut self, weight: Weight, payment: Assets) -> Result<Assets, XcmError> {
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

	fn refund_weight(&mut self, weight: Weight) -> Option<MultiAsset> {
		log::trace!(
			target: "xcm::weight", "refund_weight weight: {:?} ",
			weight
		);
		match &self.multi_location {
			None => None,
			Some(ml) => {
				if weight == 0 {
					None
				} else {
					Some((ml.clone(), weight as u128).into())
				}
			},
		}
	}
}
