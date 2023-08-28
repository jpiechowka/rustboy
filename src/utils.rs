#[inline(always)]
pub fn bool_to_bit(condition: bool) -> u8 {
    if condition {
        1
    } else {
        0
    }
}
