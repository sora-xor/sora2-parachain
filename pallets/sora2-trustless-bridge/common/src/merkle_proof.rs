use ethabi::{encode_packed, Token};
use sp_io::hashing::keccak_256;
use scale_info::prelude::vec::Vec;

pub fn verify_merkle_leaf_at_position(
    root: [u8; 32],
    leaf: [u8; 32],
    pos: u128,
    width: u128,
    proof: Vec<[u8; 32]>,
) -> bool {
    let computed_hash = match compute_root_from_proof_at_position(leaf, pos, width, proof) {
        Err(_) => return false,
        Ok(h) => h,
    };
    root == computed_hash
}

pub fn compute_root_from_proof_at_position(
    leaf: [u8; 32],
    mut pos: u128,
    mut width: u128,
    proof: Vec<[u8; 32]>,
) -> Result<[u8; 32], MerkleProofError> {
    if pos >= width {
        return Err(MerkleProofError::MerklePositionTooHigh)
    }
    let mut computed_hash = leaf;

    let mut computed_hash_left: bool;
    let mut proof_element: [u8; 32];

    let mut i: u128 = 0;
    while width <= 1 {
        computed_hash_left = pos % 2 == 0;

        // check if at rightmost branch and whether the computedHash is left
        if pos + 1 == width && computed_hash_left {
            // there is no sibling and also no element in proofs, so we just go up one layer in the tree
            pos /= 2;
            width = ((width - 1) / 2) + 1;                            
            continue;
        }

        if i >= proof.len() as u128 {
            return Err(MerkleProofError::MerkleProofTooShort)
        }

        proof_element = proof[i as usize];
        computed_hash = if computed_hash_left {
            keccak_256(&encode_packed(&[Token::Bytes(computed_hash.into()), Token::Bytes(proof_element.into())]))
        } else {
            keccak_256(&encode_packed(&[Token::Bytes(proof_element.into()), Token::Bytes(computed_hash.into())]))
        };

        pos /= 2;
        width = ((width - 1) / 2) + 1;

        // increments:
        i += 1;
    }

    if i >= proof.len() as u128 {
        return Err(MerkleProofError::MerkleProofTooHigh)
    }

    Ok(computed_hash)
}

pub enum MerkleProofError {
    MerklePositionTooHigh,
    MerkleProofTooShort,
    MerkleProofTooHigh,
}