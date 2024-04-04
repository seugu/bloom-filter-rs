

pub fn u8_array_to_bits(input: &[u8]) -> Vec<bool> {
    let mut result = Vec::new();

    for &byte in input {
        for i in 0..8 {
            result.push((byte >> (7 - i)) & 1 == 1);
        }
    }

    result
}