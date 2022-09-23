#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
// use ethereum_types::U256;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

pub struct Commitment {
	payload_prefix: Vec<u8>,
	payload: [u8; 32],
	payload_suffix: Vec<u8>,
	block_number: u32,
	validator_set_id: u64,
}

pub struct ValidatorProof<AccountId> {
	validator_claims_bitfield: Vec<u128>,
	signatures: Vec<Vec<u8>>,
	positions: Vec<u128>,
	public_keys: AccountId,
	public_key_merkle_proofs: Vec<Vec<[u8; 32]>>,
}

pub struct BeefyMMRLeaf {
	version: u8,
	parent_number: u32,
	next_authority_set_id: u64,
	next_authority_set_len: u32,
	parent_hash: [u8; 32],
	next_authority_set_root: [u8; 32],
	random_seed: [u8; 32],
	digest_hash: [u8; 32],
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	pub const MMR_ROOT_HISTORY_SIZE: u32 = 30;

	pub const THRESHOLD_NUMERATOR: u128 = 22;
	pub const THRESHOLD_DENOMINATOR: u128 = 59;

	pub const NUMBER_OF_BLOCKS_PER_SESSION: u64 = 600;
	pub const ERROR_AND_SAFETY_BUFFER: u64 = 10;
	pub const MAXIMUM_BLOCK_GAP: u64 = NUMBER_OF_BLOCKS_PER_SESSION - ERROR_AND_SAFETY_BUFFER;

	pub const MMR_ROOT_ID: [u8; 2] = [0x6d, 0x68];




	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	#[pallet::storage]
	#[pallet::getter(fn latest_mmr_roots)]
	pub type LatestMMRRoots<T> = StorageMap<_, Blake2_256, u128, [u8; 32], ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn latest_mmr_root_index)]
	pub type LatestMMRRootIndex<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn latest_beefy_block)]
	pub type LatestBeefyBlock<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn latest_random_seed)]
	pub type LatestRandomSeed<T> = StorageValue<_, u32>;

	// Validator registry storage:
	#[pallet::storage]
	#[pallet::getter(fn validator_registry_root)]
	pub type ValidatorRegistryRoot<T> = StorageValue<_, [u8; 32]>;

	#[pallet::storage]
	#[pallet::getter(fn validator_registry_num_of_validators)]
	pub type NumOfValidators<T> = StorageValue<_, u128>;

	#[pallet::storage]
	#[pallet::getter(fn latest_mmr_root_index)]
	pub type ValidatorRegistryId<T> = StorageValue<_, u64>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	impl<T: Config> Pallet<T> {
		pub fn add_known_mmr_root(root: [u8; 32]) -> u32 {
			todo!()
		}

		pub fn is_known_root() -> bool {
			todo!()
		}

		pub fn get_latest_mmr_root() {
			todo!()
		}

		pub fn verity_beefy_merkle_leaf(
			beefy_mmr_leaf: [u8; 32], /*SimplifiedMMRProof memory proof*/
		) -> bool {
			todo!()
		}

		pub fn create_random_bit_field(validator_claims_bitfield: Vec<u128>) -> Vec<u128> {
			todo!()
		}

		pub fn create_initial_bitfield(bits_to_set: Vec<u128>, length: u128) -> Vec<u128> {
			todo!()
		}

		pub fn submit_signature_commitment(
			commitment: Commitment,
			validator_proof: ValidatorProof<T::AccountId>,
			latest_mmr_leaf: BeefyMMRLeaf, /* proof: SimplifiedMMRProof*/
		) {
			todo!()
		}

		/* Private Functions */

		/*
		pub fn () -> {

		}
		*/

		pub fn get_seed() -> u128 {
			todo!()
		}

		pub fn verity_newest_mmr_leaf(
			leaf: BeefyMMRLeaf,
			root: [u8; 32], /*proof: SimplifiedMMRProof*/
		) {
			todo!()
		}

		pub fn process_payload(payload: [u8; 32], block_number: u64) {
			todo!()
		}

		pub fn apply_validator_set_changes(
			next_authority_set_id: u64,
			next_authority_set_len: u32,
			next_authority_set_root: [u8; 32],
		) {
			todo!()
		}

		pub fn get_required_number_of_signatures() -> u128 {
			todo!()
		}

		pub fn requiredNumberOfSignatures(num_validators: u128) -> u128 {
			todo!()
		}

		/**
			* @dev https://github.com/sora-xor/substrate/blob/7d914ce3ed34a27d7bb213caed374d64cde8cfa8/client/beefy/src/round.rs#L62
			*/
		pub fn check_commitment_signatures_threshold(
			num_of_validators: u128,
			validator_claims_bitfield: Vec<u128>,
		) {
			todo!()
		}

		pub fn verify_commitment(commitment: Commitment, proof: ValidatorProof) {
			todo!()
		}

		pub fn verify_validator_proof_lengths(
			required_num_of_signatures: u128,
			proof: ValidatorProof,
		) {
			todo!()
		}

		pub fn verify_validator_proof_signatures(
			random_bitfield: [u8; 32],
			proof: ValidatorProof,
			required_num_of_signatures: u128,
			commitment_hash: [u8; 32],
		) {
			todo!()
		}

		pub fn verify_validator_signature(
			random_bitfield: [u8; 32],
			signature: Vec<u8>,
			position: u128,
			public_key: T::AccountId,
			public_key_merkle_proof: Vec<[u8; 32]>,
			commitment_hash: [u8; 32],
		) {
			todo!()
		}

		pub fn create_commitment_hash(commitment: Commitment) -> () {
			todo!()
		}

		pub fn encode_mmr_leaf(leaf: BeefyMMRLeaf) -> Vec<u8> {
			todo!()
		}

		pub fn hash_mmr_leaf() -> [u8; 32] {
			todo!()
		}

		pub fn validator_registry_update(new_root: [u8; 32], new_num_of_validators: u128, new_id: u64){
			todo!()
		}

		pub fn check_validator_in_set(addr: T::AccountId, pos: u128, proof: [u8; 32]) -> bool {
			todo!()
		}
	}
}
