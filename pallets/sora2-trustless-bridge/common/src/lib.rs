pub mod simplified_mmr_proof;
pub mod bitfield;
pub mod merkle_proof;


pub fn concat_u8(first: &[u8], second: &[u8]) -> Vec<u8> {
	[first, second].concat()
}