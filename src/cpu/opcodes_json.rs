// https://gbdev.io/gb-opcodes/optables
// https://gbdev.io/gb-opcodes/Opcodes.json

use std::collections::HashMap;

use log::info;
use serde::{Deserialize, Serialize};

// TODO: check visibility on structs and functions

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Opcodes {
    pub unprefixed: HashMap<String, OpcodeDetails>,
    pub cbprefixed: HashMap<String, OpcodeDetails>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OpcodeDetails {
    pub mnemonic: String, // Illegal opcodes are prefixed with ILLEGAL, (for example: ILLEGAL_D3 from unprefixed)
    pub bytes: u8,
    pub cycles: Vec<u8>,
    pub operands: Vec<Operand>,
    pub immediate: bool,
    pub flags: Flags,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Operand {
    pub name: String,
    #[serde(default)]
    pub bytes: u8,
    pub immediate: bool,
}

// Z - Zero Flag
// N - Subtract Flag
// H - Half Carry Flag
// C - Carry Flag
// 0 - The flag is reset
// 1 - The flag is set
// - - The flag is left untouched
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Flags {
    #[serde(rename = "Z")]
    pub zero: String,
    #[serde(rename = "N")]
    pub subtract: String,
    #[serde(rename = "H")]
    pub half_carry: String,
    #[serde(rename = "C")]
    pub carry: String,
}

impl Opcodes {
    pub fn new() -> Opcodes {
        info!("Loading opcodes from included JSON");
        let json_opcodes = include_str!("../opcodes/opcodes.json");
        let opcodes: Opcodes =
            serde_json::from_str(json_opcodes).expect("should load opcodes form included JSON");

        info!("Opcodes loaded successfully and ready to use");
        Opcodes {
            unprefixed: opcodes.unprefixed,
            cbprefixed: opcodes.cbprefixed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_init_opcodes() {
        let opcodes = Opcodes::new();
        assert_eq!(opcodes.unprefixed.len(), 256);
        assert_eq!(opcodes.cbprefixed.len(), 256);

        let unprefixed_0xCE_opcode = opcodes.unprefixed.get("0xCE").unwrap();
        assert_eq!(unprefixed_0xCE_opcode.mnemonic, "ADC");
        assert_eq!(unprefixed_0xCE_opcode.bytes, 2);
        assert_eq!(unprefixed_0xCE_opcode.cycles, vec![8]);
        assert_eq!(
            unprefixed_0xCE_opcode.operands,
            vec![
                Operand {
                    name: "A".to_string(),
                    bytes: 0,
                    immediate: true,
                },
                Operand {
                    name: "n8".to_string(),
                    bytes: 1,
                    immediate: true,
                }
            ]
        );
        assert!(unprefixed_0xCE_opcode.immediate);
        assert_eq!(unprefixed_0xCE_opcode.flags.zero, "Z");
        assert_eq!(unprefixed_0xCE_opcode.flags.subtract, "0");
        assert_eq!(unprefixed_0xCE_opcode.flags.half_carry, "H");
        assert_eq!(unprefixed_0xCE_opcode.flags.carry, "C");

        let cbprefixed_0x00_opcode = opcodes.cbprefixed.get("0x00").unwrap();
        assert_eq!(cbprefixed_0x00_opcode.mnemonic, "RLC");
        assert_eq!(cbprefixed_0x00_opcode.bytes, 2);
        assert_eq!(cbprefixed_0x00_opcode.cycles, vec![8]);
        assert_eq!(
            cbprefixed_0x00_opcode.operands,
            vec![Operand {
                name: "B".to_string(),
                bytes: 0,
                immediate: true,
            },]
        );
        assert!(cbprefixed_0x00_opcode.immediate);
        assert_eq!(cbprefixed_0x00_opcode.flags.zero, "Z");
        assert_eq!(cbprefixed_0x00_opcode.flags.subtract, "0");
        assert_eq!(cbprefixed_0x00_opcode.flags.half_carry, "0");
        assert_eq!(cbprefixed_0x00_opcode.flags.carry, "C");
    }

    #[test]
    fn can_deserialize_unprefixed_opcode_details_from_json() {
        let json = r#"
        {
			"mnemonic": "ADC",
			"bytes": 2,
			"cycles": [
				8
			],
			"operands": [
				{
					"name": "A",
					"immediate": true
				},
				{
					"name": "n8",
					"bytes": 1,
					"immediate": true
				}
			],
			"immediate": true,
			"flags": {
				"Z": "Z",
				"N": "0",
				"H": "H",
				"C": "C"
			}
		}
    "#;

        let opcode: OpcodeDetails = serde_json::from_str(json).unwrap();

        assert_eq!(opcode.mnemonic, "ADC");
        assert_eq!(opcode.bytes, 2);
        assert_eq!(opcode.cycles, vec![8]);
        assert_eq!(
            opcode.operands,
            vec![
                Operand {
                    name: "A".to_string(),
                    bytes: 0,
                    immediate: true,
                },
                Operand {
                    name: "n8".to_string(),
                    bytes: 1,
                    immediate: true,
                }
            ]
        );
        assert!(opcode.immediate);
        assert_eq!(opcode.flags.zero, "Z");
        assert_eq!(opcode.flags.subtract, "0");
        assert_eq!(opcode.flags.half_carry, "H");
        assert_eq!(opcode.flags.carry, "C");
    }

    #[test]
    fn can_deserialize_prefixed_opcode_details_from_json() {
        let json = r#"
        {
			"mnemonic": "RLC",
			"bytes": 2,
			"cycles": [
				8
			],
			"operands": [
				{
					"name": "B",
					"immediate": true
				}
			],
			"immediate": true,
			"flags": {
				"Z": "Z",
				"N": "0",
				"H": "0",
				"C": "C"
			}
		}
    "#;

        let opcode: OpcodeDetails = serde_json::from_str(json).unwrap();

        assert_eq!(opcode.mnemonic, "RLC");
        assert_eq!(opcode.bytes, 2);
        assert_eq!(opcode.cycles, vec![8]);
        assert_eq!(
            opcode.operands,
            vec![Operand {
                name: "B".to_string(),
                bytes: 0,
                immediate: true,
            },]
        );
        assert!(opcode.immediate);
        assert_eq!(opcode.flags.zero, "Z");
        assert_eq!(opcode.flags.subtract, "0");
        assert_eq!(opcode.flags.half_carry, "0");
        assert_eq!(opcode.flags.carry, "C");
    }
}
