
//! Autogenerated weights for `pallet_converter`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-20, STEPS: `5`, REPEAT: 1, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/parachain-collator
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_converter
// --extrinsic
// *
// --steps
// 5
// --repeat
// 1
// --output
// pallets/converter/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;
use parachain_common::primitives::EXTRINSIC_FIXED_WEIGHT;

/// Weight functions for `pallet_converter`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfo for WeightInfo<T> {
	// Storage: Converter AssetIdToMultilocation (r:1 w:1)
	// Storage: Converter MultilocationToAssetId (r:0 w:1)
	fn register_mapping() -> Weight {
		Weight::from_ref_time(25_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Converter AssetIdToMultilocation (r:1 w:1)
	// Storage: Converter MultilocationToAssetId (r:1 w:2)
	fn change_asset_mapping() -> Weight {
		Weight::from_ref_time(22_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Converter MultilocationToAssetId (r:1 w:1)
	// Storage: Converter AssetIdToMultilocation (r:1 w:2)
	fn change_multilocation_mapping() -> Weight {
		Weight::from_ref_time(21_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Converter AssetIdToMultilocation (r:1 w:1)
	// Storage: Converter MultilocationToAssetId (r:0 w:1)
	fn delete_mapping() -> Weight {
		Weight::from_ref_time(17_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}

	fn transfer() -> Weight {
		Weight::from_ref_time(17_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
    }

	fn register_asset() -> Weight {
		Weight::from_ref_time(17_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
    }
}


impl crate::WeightInfo for () {
	fn register_mapping() -> Weight {
		EXTRINSIC_FIXED_WEIGHT
	}

	fn change_asset_mapping() -> Weight {
		EXTRINSIC_FIXED_WEIGHT
	}

	fn change_multilocation_mapping() -> Weight {
		EXTRINSIC_FIXED_WEIGHT
	}

	fn delete_mapping() -> Weight {
		EXTRINSIC_FIXED_WEIGHT
	}

	fn transfer() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }

	fn register_asset() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
}
