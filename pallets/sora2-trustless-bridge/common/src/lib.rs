#![cfg_attr(not(feature = "std"), no_std)]

pub mod simplified_mmr_proof;
pub mod bitfield;
pub mod merkle_proof;

use scale_info::prelude::vec::Vec;

pub fn concat_u8(slice: &[&[u8]]) -> Vec<u8> {
	slice.concat()
}