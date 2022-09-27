pub mod simplified_mmr_proof;
pub mod bitfield;
pub mod merkle_proof;

pub fn concat_u8(slice: &[&[u8]]) -> Vec<u8> {
	slice.concat()
}