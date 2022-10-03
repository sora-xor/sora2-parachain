pub const SIZE: u128 = core::mem::size_of::<u128>() as u128;

pub fn create_bitfield(bits_to_set: Vec<u128>, length: u128) -> Vec<u128> {
    let array_length = (length + 255) / 256;
    let mut bitfield = Vec::with_capacity(array_length as usize);
    for i in 0..bits_to_set.len() {
        set(&mut bitfield, bits_to_set[i])
    }
    bitfield
}

pub fn set(self_val: &mut Vec<u128>, index: u128)  {
    let element = index / SIZE;
    let within = (index % SIZE) as u8;
    // unsafe casting!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    self_val[element as usize] = self_val[element as usize] | 1 << within;
}

pub fn is_set(self_val: &Vec<u128>, index: u128) -> bool {
    let element = index/SIZE;
    let within = (index % SIZE) as u8;
    self_val[element as usize] >> within & 1 == 1
}

pub fn clear(self_val: &mut Vec<u128>, index: u128) {
    let element = index/SIZE;
    let within = (index % SIZE) as u8;
    self_val[element as usize] = self_val[element as usize] & !(1 << within);
}

#[cfg(test)]
mod test {
    #[test]
    fn is_set_returns_ok() {
        let a = 0b0010 as u8;
        assert_eq!(a >> 1 & 1, 1)
        // assert_eq!(a & 1 << 1, 1)
    }
}