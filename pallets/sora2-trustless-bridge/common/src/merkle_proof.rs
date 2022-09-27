

pub fn verify_merkle_leaf_at_position(
    root: [u8; 32],
    leaf: [u8; 32],
    pos: u128,
    width: u128,
    proof: Vec<[u8; 32]>,
) -> bool {
    let computed_hash = compute_root_from_proof_at_position(leaf, pos, width, proof);
    root == computed_hash
}

pub fn compute_root_from_proof_at_position(
    leaf: [u8; 32],
    pos: u128,
    width: u128,
    proof: Vec<[u8; 32]>,
) -> [u8; 32] {
    todo!()
}