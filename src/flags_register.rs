#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlagsRegister {
    // set to true if the result of the operation is equal to 0. Used by conditional jumps
    pub zero: bool,
    // set to true if the operation was a subtraction
    pub subtract: bool,
    // set to true if there is an overflow from the lower nibble (a.k.a the lower four bits) to the upper nibble (a.k.a the upper four bits)
    pub half_carry: bool,
    // set to true if the operation resulted in an overflow
    pub carry: bool,
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl FlagsRegister {
    pub fn new() -> Self {
        Self {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        }
    }
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_to_u8_expected_bits_1111_0000() {
        let mut flags = FlagsRegister::new();
        flags.zero = true;
        flags.subtract = true;
        flags.half_carry = true;
        flags.carry = true;
        let result: u8 = flags.into();
        assert_eq!(result, 0b1111_0000u8);
    }

    #[test]
    fn conversion_to_u8_expected_bits_1110_0000() {
        let mut flags = FlagsRegister::new();
        flags.zero = true;
        flags.subtract = true;
        flags.half_carry = true;
        let result: u8 = flags.into();
        assert_eq!(result, 0b1110_0000u8);
    }

    #[test]
    fn conversion_to_u8_expected_bits_1100_0000() {
        let mut flags = FlagsRegister::new();
        flags.zero = true;
        flags.subtract = true;
        let result: u8 = flags.into();
        assert_eq!(result, 0b1100_0000u8);
    }

    #[test]
    fn conversion_to_u8_expected_bits_1000_0000() {
        let mut flags = FlagsRegister::new();
        flags.zero = true;
        let result: u8 = flags.into();
        assert_eq!(result, 0b1000_0000u8);
    }

    #[test]
    fn conversion_to_u8_expected_bits_0000_0000() {
        let flags = FlagsRegister::new();
        let result: u8 = flags.into();
        assert_eq!(result, 0b0000_0000u8);
    }

    #[test]
    fn conversion_to_u8_expected_bits_0110_0000() {
        let mut flags = FlagsRegister::new();
        flags.subtract = true;
        flags.half_carry = true;
        let result: u8 = flags.into();
        assert_eq!(result, 0b0110_0000u8);
    }

    #[test]
    fn conversion_to_u8_expected_bits_1001_0000() {
        let mut flags = FlagsRegister::new();
        flags.zero = true;
        flags.carry = true;
        let result: u8 = flags.into();
        assert_eq!(result, 0b1001_0000u8);
    }

    #[test]
    fn conversion_from_u8_bits_1111_0000() {
        let result: FlagsRegister = 0b1111_0000.into();
        assert!(result.zero);
        assert!(result.subtract);
        assert!(result.half_carry);
        assert!(result.carry);
    }

    #[test]
    fn conversion_from_u8_bits_1110_0000() {
        let result: FlagsRegister = 0b1110_0000.into();
        assert!(result.zero);
        assert!(result.subtract);
        assert!(result.half_carry);
        assert!(!result.carry);
    }

    #[test]
    fn conversion_from_u8_bits_1100_0000() {
        let result: FlagsRegister = 0b1100_0000.into();
        assert!(result.zero);
        assert!(result.subtract);
        assert!(!result.half_carry);
        assert!(!result.carry);
    }

    #[test]
    fn conversion_from_u8_bits_1000_0000() {
        let result: FlagsRegister = 0b1000_0000.into();
        assert!(result.zero);
        assert!(!result.subtract);
        assert!(!result.half_carry);
        assert!(!result.carry);
    }

    #[test]
    fn conversion_from_u8_bits_0000_0000() {
        let result: FlagsRegister = 0b0000_0000.into();
        assert!(!result.zero);
        assert!(!result.subtract);
        assert!(!result.half_carry);
        assert!(!result.carry);
    }

    #[test]
    fn conversion_from_u8_bits_0110_0000() {
        let result: FlagsRegister = 0b0110_0000.into();
        assert!(!result.zero);
        assert!(result.subtract);
        assert!(result.half_carry);
        assert!(!result.carry);
    }

    #[test]
    fn conversion_from_u8_bits_1001_0000() {
        let result: FlagsRegister = 0b1001_0000.into();
        assert!(result.zero);
        assert!(!result.subtract);
        assert!(!result.half_carry);
        assert!(result.carry);
    }
}
