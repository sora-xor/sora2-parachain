pub fn create_bitfield(bits_to_set: Vec<u128>, length: u128) -> Vec<u128> {
    let array_length = (length + 255) / 256;
    let mut bitfield = Vec::new();
    for i in 0..bits_to_set.len() {
        set(&mut bitfield, bits_to_set[i])
    }
    bitfield
}

pub fn set(self_val: &mut Vec<u128>, index: u128) {
    let element = index / 256;
    let within = (index % 256) as u8;
    // unsafe casting!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    self_val[element as usize] = self_val[element as usize]
}