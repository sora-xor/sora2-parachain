use xcm_executor::{
	traits::{DropAssets, WeightTrader},
	Assets,
};
use xcm_builder::TakeRevenue;
use frame_support::{
	traits::Get,
	weights::{constants::WEIGHT_PER_SECOND, Weight},
};
use sp_std::{marker::PhantomData, prelude::*};
use xcm::latest::prelude::*;


pub struct ParachainTrader {
    pub weight: Weight
}

impl WeightTrader for ParachainTrader {
    fn new() -> Self {
		Self {
			weight: 0,
		}
	}

    fn buy_weight(&mut self, weight: Weight, payment: Assets) -> Result<Assets, XcmError> {
		log::trace!(target: "xcm::weight", "buy_weight weight: {:?}, payment: {:?}", weight, payment);


		log::trace!(target: "xcm::weight", "no concrete fungible asset");
		Err(XcmError::TooExpensive)
	}

    fn refund_weight(&mut self, weight: Weight) -> Option<MultiAsset> {
		log::trace!(
			target: "xcm::weight", "refund_weight weight: {:?} ",
			weight
		);
        None
	}
}