#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
use xcm::latest::prelude::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use xcm::latest::prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type AssetId;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type AssetIdToMultilocation<T: Config> =
		StorageMap<_, Blake2_256, [u8; 32], MultiLocation, OptionQuery>;

	#[pallet::storage]
	pub type MultilocationToAssetId<T: Config> =
		StorageMap<_, Blake2_256, MultiLocation, [u8; 32], OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		// /// Error names should be descriptive.
		// NoneValue,
		// /// Errors should have helpful documentation associated with them.
		// StorageOverflow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(0)]
		pub fn register_pair(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn change_pair(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn delete_pair(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		// pub fn get_mulilocation(asset_id: [u8;32]) -> Option<MultiLocation>{

		// }
	}


	impl<T> sp_runtime::traits::Convert<[u8; 32], Option<MultiLocation>> for Pallet<T> {
		fn convert(id: [u8; 32]) -> Option<MultiLocation> {
			None
		}
	}

	impl<T> sp_runtime::traits::Convert<MultiLocation, Option<[u8; 32]>> for Pallet<T> {
		fn convert(multilocation: MultiLocation) -> Option<[u8; 32]> {
			None
		}
}
}


