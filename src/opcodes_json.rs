// https://gbdev.io/gb-opcodes/optables
// https://gbdev.io/gb-opcodes/Opcodes.json

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Opcode {
    flags: Flags,
}

#[derive(Serialize, Deserialize, Debug)]
struct Flags {
    #[serde(rename = "Z")]
    zero: String,
    #[serde(rename = "N")]
    subtract: String,
    #[serde(rename = "H")]
    half_carry: String,
    #[serde(rename = "C")]
    carry: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_opcode_from_json() {
        let json = r#"
        {
            "mnemonic": "DEC",
            "bytes": 1,
            "cycles": [4],
            "operands": [
                {
                    "name": "B",
                    "immediate": true
                }
            ],
            "immediate": true,
            "flags": {
                "Z": "Z",
                "N": "1",
                "H": "H",
                "C": "-"
            }
        }
    "#;

        let opcode: Opcode = serde_json::from_str(json).unwrap();
        println!("{:?}", opcode)
    }
}
