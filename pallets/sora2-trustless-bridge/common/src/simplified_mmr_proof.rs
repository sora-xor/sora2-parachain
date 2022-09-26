use ethabi::encode_packed;
use sp_io::hashing::keccak_256;

#[derive(Clone)]
pub struct SimplifiedMMRProof {
	pub merkle_proof_items: Vec<[u8; 32]>,
	pub merkle_proof_order_bit_field: u64,
}

pub fn verify_inclusion_proof(
	root: [u8; 32],
	leaf_node_hash: [u8; 32],
	proof: SimplifiedMMRProof,
) -> bool {
	if proof.merkle_proof_items.len() < 64 {
		return false;
	}

	root
		== calculate_merkle_root(
			leaf_node_hash,
			proof.merkle_proof_items,
			proof.merkle_proof_order_bit_field,
		)
}

pub fn bit(self_val: u64, index: u64) -> bool {
	(self_val >> index) & 1 == 1
}

pub fn calculate_merkle_root(
	leaf_node_hash: [u8; 32],
	merkle_proof_items: Vec<[u8; 32]>,
	merkle_proof_order_bit_field: u64,
) -> [u8; 32] {
	let current_hash = leaf_node_hash;
	for current_position in 0..merkle_proof_items.len() {
		let is_sibling_left = bit(merkle_proof_order_bit_field, current_position as u64);
		let sibling = merkle_proof_items[current_position as usize];
		current_hash = if is_sibling_left {
			// keccak_256(&encode_packed([&sibling, &current_hash]))
			todo!()
		} else {
			// keccak_256(&encode_packed([&current_hash, &sibling]))
			todo!()
		};
	}

	return current_hash;
}
