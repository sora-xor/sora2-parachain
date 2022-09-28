#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[derive(
	Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord, scale_info::TypeInfo,
)]
pub struct Commitment {
	pub payload_prefix: Vec<u8>,
	pub payload: [u8; 32],
	pub payload_suffix: Vec<u8>,
	pub block_number: u32,
	pub validator_set_id: u64,
}

#[derive(
	Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord, scale_info::TypeInfo,
)]
pub struct ValidatorProof<AccountId> {
	pub validator_claims_bitfield: Vec<u128>,
	pub signatures: Vec<Vec<u8>>,
	pub positions: Vec<u128>,
	pub public_keys: Vec<AccountId>,
	pub public_key_merkle_proofs: Vec<Vec<[u8; 32]>>,
}

#[derive(
	Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord, scale_info::TypeInfo,
)]
pub struct BeefyMMRLeaf {
	pub version: u8,
	pub parent_number: u32,
	pub next_authority_set_id: u64,
	pub next_authority_set_len: u32,
	pub parent_hash: [u8; 32],
	pub next_authority_set_root: [u8; 32],
	pub random_seed: [u8; 32],
	pub digest_hash: [u8; 32],
}

#[frame_support::pallet]
pub mod pallet {
	use super::{BeefyMMRLeaf, Commitment, ValidatorProof};
	use common::{bitfield, merkle_proof, simplified_mmr_proof::*};
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_io::hashing::keccak_256;

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
	pub type LatestMMRRootIndex<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn latest_beefy_block)]
	pub type LatestBeefyBlock<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn latest_random_seed)]
	pub type LatestRandomSeed<T> = StorageValue<_, [u8; 32], ValueQuery>;

	// Validator registry storage:
	#[pallet::storage]
	#[pallet::getter(fn validator_registry_root)]
	pub type ValidatorRegistryRoot<T> = StorageValue<_, [u8; 32], ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn validator_registry_num_of_validators)]
	pub type NumOfValidators<T> = StorageValue<_, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn validator_registry_id)]
	pub type ValidatorRegistryId<T> = StorageValue<_, u64, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		VerificationSuccessful(T::AccountId, u32),
		NewMMRRoot([u8; 32], u64),
		ValidatorRegistryUpdated([u8; 32], u128, u64),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		InvalidValidatorSetId,
		InvalidMMRProof,
		PayloadBlocknumberTooOld,
		PayloadBlocknumberTooNew,
		CannotSwitchOldValidatorSet,
		NotEnoughValidatorSignatures,
		InvalidNumberOfSignatures,
		InvalidNumberOfPositions,
		InvalidNumberOfPublicKeys,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	impl<T: Config> Pallet<T> {
		pub fn add_known_mmr_root(root: [u8; 32]) -> u32 {
			let latest_mmr_root_index = LatestMMRRootIndex::<T>::get();
			let new_root_index = (latest_mmr_root_index + 1) % MMR_ROOT_HISTORY_SIZE;
			LatestMMRRoots::<T>::insert(latest_mmr_root_index as u128, root);
			LatestMMRRootIndex::<T>::set(latest_mmr_root_index);
			latest_mmr_root_index
		}

		pub fn is_known_root(root: [u8; 32]) -> bool {
			if root == <[u8; 32] as Default>::default() {
				return false;
			}
			let latest_mmr_root_index = LatestMMRRootIndex::<T>::get();
			let mut i = latest_mmr_root_index;
			let latest_mmr_roots = LatestMMRRoots::<T>::get(i as u128);
			loop {
				if root == LatestMMRRoots::<T>::get(i as u128) {
					return true;
				}
				if i == 0 {
					i = MMR_ROOT_HISTORY_SIZE
				}
				i = i - 1;
				if i != latest_mmr_root_index {
					break;
				}
			}
			false
		}

		pub fn get_latest_mmr_root() -> [u8; 32] {
			LatestMMRRoots::<T>::get(LatestMMRRootIndex::<T>::get() as u128)
		}

		pub fn verity_beefy_merkle_leaf(
			beefy_mmr_leaf: [u8; 32],
			proof: SimplifiedMMRProof,
		) -> bool {
			let proof_root = calculate_merkle_root(
				beefy_mmr_leaf,
				proof.merkle_proof_items,
				proof.merkle_proof_order_bit_field,
			);
			Self::is_known_root(proof_root)
		}

		pub fn create_random_bit_field(validator_claims_bitfield: Vec<u128>) -> Vec<u128> {
			todo!()
		}

		pub fn create_initial_bitfield(bits_to_set: Vec<u128>, length: u128) -> Vec<u128> {
			bitfield::create_bitfield(bits_to_set, length)
		}

		pub fn submit_signature_commitment(
			origin: OriginFor<T>,
			commitment: Commitment,
			validator_proof: ValidatorProof<T::AccountId>,
			latest_mmr_leaf: BeefyMMRLeaf,
			proof: SimplifiedMMRProof,
		) -> DispatchResultWithPostInfo {
			let signer = ensure_signed(origin)?;
			ensure!(
				commitment.validator_set_id == Self::validator_registry_id(),
				Error::<T>::InvalidValidatorSetId
			);
			Self::verify_commitment(&commitment, &validator_proof)?;
			Self::verity_newest_mmr_leaf(&latest_mmr_leaf, &commitment.payload, &proof)?;
			Self::process_payload(&commitment.payload, commitment.block_number.into())?;

			LatestRandomSeed::<T>::set(latest_mmr_leaf.random_seed);

			Self::deposit_event(Event::VerificationSuccessful(signer, commitment.block_number));
			Self::apply_validator_set_changes(
				latest_mmr_leaf.next_authority_set_id,
				latest_mmr_leaf.next_authority_set_len,
				latest_mmr_leaf.next_authority_set_root,
			)?;
			Ok(().into())
		}

		/* Private Functions */
		pub fn get_seed() -> [u8; 32] {
			let concated = common::concat_u8(&[
				&Self::latest_random_seed(),
				&Self::latest_beefy_block().to_be_bytes(),
			]);
			keccak_256(&concated)
		}

		pub fn verity_newest_mmr_leaf(
			leaf: &BeefyMMRLeaf,
			root: &[u8; 32],
			proof: &SimplifiedMMRProof,
		) -> DispatchResultWithPostInfo {
			let encoded_leaf = Self::encode_mmr_leaf(leaf.clone());
			let hash_leaf = Self::hash_mmr_leaf(encoded_leaf);
			ensure!(
				verify_inclusion_proof(*root, hash_leaf, proof.clone()),
				Error::<T>::InvalidMMRProof
			);
			Ok(().into())
		}

		// TODO!!!!!
		// u64 casting to u32!!!!!!!
		pub fn process_payload(
			payload: &[u8; 32],
			block_number: u64,
		) -> DispatchResultWithPostInfo {
			ensure!(
				block_number > Self::latest_beefy_block() as u64,
				Error::<T>::PayloadBlocknumberTooOld
			);
			ensure!(
				block_number < Self::latest_beefy_block() as u64 + MAXIMUM_BLOCK_GAP,
				Error::<T>::PayloadBlocknumberTooNew
			);
			Self::add_known_mmr_root(*payload);
			// NOT SAFE!!!!!!!!!
			LatestBeefyBlock::<T>::set(block_number.try_into().unwrap());
			Self::deposit_event(Event::NewMMRRoot(*payload, block_number));
			Ok(().into())
		}

		pub fn apply_validator_set_changes(
			next_authority_set_id: u64,
			next_authority_set_len: u32,
			next_authority_set_root: [u8; 32],
		) -> DispatchResultWithPostInfo {
			if next_authority_set_id > Self::validator_registry_id() {
				ensure!(
					next_authority_set_id > Self::validator_registry_id(),
					Error::<T>::CannotSwitchOldValidatorSet
				);
				Self::validator_registry_update(
					next_authority_set_root,
					next_authority_set_len as u128,
					next_authority_set_id,
				);
			}
			Ok(().into())
		}

		pub fn required_number_of_signatures() -> u128 {
			Self::get_required_number_of_signatures(Self::validator_registry_num_of_validators())
		}

		pub fn get_required_number_of_signatures(num_validators: u128) -> u128 {
			(num_validators * THRESHOLD_NUMERATOR + THRESHOLD_DENOMINATOR - 1)
				/ THRESHOLD_DENOMINATOR
		}

		/**
			* @dev https://github.com/sora-xor/substrate/blob/7d914ce3ed34a27d7bb213caed374d64cde8cfa8/client/beefy/src/round.rs#L62
			*/
		pub fn check_commitment_signatures_threshold(
			num_of_validators: u128,
			validator_claims_bitfield: Vec<u128>,
		) -> DispatchResultWithPostInfo {
			let threshold = num_of_validators - (num_of_validators - 1) / 3;
			todo!("finish");
			// ensure!(bitgield::count_set_bits(validator_claims_bitfield) >= threshold, Error::<T>::NotEnoughValidatorSignatures);
			Ok(().into())
		}

		pub fn verify_commitment(
			commitment: &Commitment,
			proof: &ValidatorProof<T::AccountId>,
		) -> DispatchResultWithPostInfo {
			todo!()
		}

		pub fn verify_validator_proof_lengths(
			required_num_of_signatures: u128,
			proof: ValidatorProof<T::AccountId>,
		) -> DispatchResultWithPostInfo {
			ensure!(
				proof.signatures.len() as u128 == required_num_of_signatures,
				Error::<T>::InvalidNumberOfSignatures
			);
			ensure!(
				proof.positions.len() as u128 == required_num_of_signatures,
				Error::<T>::InvalidNumberOfPositions
			);
			ensure!(
				proof.public_keys.len() as u128 == required_num_of_signatures,
				Error::<T>::InvalidNumberOfPublicKeys
			);
			ensure!(
				proof.public_key_merkle_proofs.len() as u128 == required_num_of_signatures,
				Error::<T>::InvalidNumberOfPublicKeys
			);
			Ok(().into())
		}

		pub fn verify_validator_proof_signatures(
			random_bitfield: [u8; 32],
			proof: ValidatorProof<T::AccountId>,
			required_num_of_signatures: u128,
			commitment_hash: [u8; 32],
		) -> DispatchResultWithPostInfo {
			let required_num_of_signatures = required_num_of_signatures as usize;
			for i in 0..required_num_of_signatures {
				Self::verify_validator_signature(
					random_bitfield,
					proof.signatures[i].clone(),
					proof.positions[i],
					proof.public_keys[i].clone(),
					proof.public_key_merkle_proofs[i].clone(),
					commitment_hash,
				)?;
			}
			Ok(().into())
		}

		pub fn verify_validator_signature(
			random_bitfield: [u8; 32],
			signature: Vec<u8>,
			position: u128,
			public_key: T::AccountId,
			public_key_merkle_proof: Vec<[u8; 32]>,
			commitment_hash: [u8; 32],
		) -> DispatchResultWithPostInfo {
			todo!()
		}

		pub fn create_commitment_hash(commitment: Commitment) -> () {
			todo!()
		}

		pub fn encode_mmr_leaf(leaf: BeefyMMRLeaf) -> Vec<u8> {
			leaf.encode()
		}

		pub fn hash_mmr_leaf(leaf: Vec<u8>) -> [u8; 32] {
			keccak_256(&leaf)
		}

		pub fn validator_registry_update(
			new_root: [u8; 32],
			new_num_of_validators: u128,
			new_id: u64,
		) {
			ValidatorRegistryRoot::<T>::set(new_root);
			NumOfValidators::<T>::set(new_num_of_validators);
			ValidatorRegistryId::<T>::set(new_id);
			Self::deposit_event(Event::<T>::ValidatorRegistryUpdated(
				new_root,
				new_num_of_validators,
				new_id,
			));
		}

		pub fn check_validator_in_set(addr: T::AccountId, pos: u128, proof: Vec<[u8; 32]>) -> bool {
			let hashed_leaf = keccak_256(&addr.encode());
			merkle_proof::verify_merkle_leaf_at_position(
				Self::validator_registry_root(),
				hashed_leaf,
				pos,
				Self::validator_registry_num_of_validators(),
				proof,
			)
		}
	}
}
