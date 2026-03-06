use std::io::{self, Read};
use crypto_bigint::U256;

use crate::{algorithms::endian::little_endian_to_u64, bitcoin::{utils::read_bytes, varint::{encode_varint, read_varint}}};

#[derive(Debug)]
pub struct Script{
    pub cmds: Vec<Vec<u8>>
}

impl Script {

    pub fn new(cmds: Vec<Vec<u8>>) -> Self {
        Self { cmds }
    }

    pub fn parse<R: Read>(stream: &mut R) -> io::Result<Script> {
        let len = read_varint(stream)?;
        let mut cmds: Vec<Vec<u8>> = Vec::new();
        let mut count = 0u64;

        while count < len {
            let mut current= [0u8; 1];
            stream.read_exact(&mut current)?;
            count += 1;

            let opcode = current[0];

            if opcode >= 1 && opcode <= 75 {
                let mut data  = vec![0u8; opcode as usize];
                stream.read_exact(&mut data)?;
                cmds.push(data);
                count += opcode as u64;
            } else if opcode == 76 { //OP_PUSHDATA1
                let next_byte = read_bytes(stream, 1)?;
                let data_len = little_endian_to_u64(&next_byte);

                let mut data  = vec![0u8; data_len as usize];
                stream.read_exact(&mut data)?;
                cmds.push(data);
                count += data_len as u64 + 1u64;
            } else if opcode == 77 { //OP_PUSHDATA2
                let next_bytes = read_bytes(stream, 2)?;
                let data_len = little_endian_to_u64(&next_bytes);

                let mut data  = vec![0u8; data_len as usize];
                stream.read_exact(&mut data)?;
                cmds.push(data);
                count += data_len as u64 + 2u64;
            } else {
                cmds.push(vec![opcode]);
            }
        }

        if count != len {
            panic!("Parsing script failed");
        }

        // println!("{cmds:?}");

        Ok(Script::new(cmds))
    }

    fn raw_serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        for cmd in &self.cmds {
            let cmd_len = cmd.len() as u16;
            if cmd_len == 1 { // Op code
                result.extend_from_slice(&cmd);
            } else {
                if cmd_len < 75 {
                    let len = cmd_len.to_le_bytes()[0];
                    result.push(len);
                } else if cmd_len >= 76 && cmd_len < 0x100 {
                    result.extend_from_slice(&76u8.to_le_bytes());
                    let len = cmd_len.to_le_bytes()[0];
                    result.push(len);
                } else if cmd_len >= 0x100 && cmd_len < 520  {
                    result.extend_from_slice(&77u8.to_le_bytes());
                    let len = cmd_len.to_le_bytes()[0];
                    result.push(len);
                } else {
                    panic!("too long an cmd");
                }
                result.extend_from_slice(cmd);
            }
        }
        result
    }

    pub fn serialize(&self) -> Vec<u8> {
        let result = self.raw_serialize();
        let total = U256::from_u64(result.len() as u64);
        let mut total_encoded = encode_varint(total);
        // println!("result: {result:?}");
        // println!("total_encoded: {total_encoded:?}");
        total_encoded.extend_from_slice(&result);
        total_encoded
        // result
    }

    pub fn add(&mut self, other: &Script) -> Script {
        let mut result: Vec<Vec<u8>> = Vec::new();
        result.extend_from_slice(&self.cmds);
        result.extend_from_slice(&other.cmds);
        Script::new(result)
    }

}

#[cfg(test)]
mod script_tests {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn parse_test() -> Result<(), Box<dyn std::error::Error>> {
        let script_pubkey_bytes = hex::decode("43410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac")?;
        let mut stream = Cursor::new(script_pubkey_bytes);
        let script = Script::parse(&mut stream)?;

        assert_eq!(script.cmds[0].len(), 65);
        assert_eq!(script.cmds[1], Vec::from([172]));
        // print!("script: {script:?}");
        Ok(())
    }

    #[test]
    fn serialize_test() -> Result<(), Box<dyn std::error::Error>> {
        let cmds: Vec<Vec<u8>> = Vec::from([
            Vec::from([4, 17, 219, 147, 225, 220, 219, 138, 1, 107, 73, 132, 15, 140, 83, 188, 30, 182, 138, 56, 46, 151, 177, 72, 46, 202, 215, 177, 72, 166, 144, 154, 92, 178, 224, 234, 221, 251, 132, 204, 249, 116, 68, 100, 248, 46, 22, 11, 250, 155, 139, 100, 249, 212, 192, 63, 153, 155, 134, 67, 246, 86, 180, 18, 163]),
            Vec::from([172]),
        ]);
        let script = Script::new(cmds);

        let serialized = script.serialize();
        let hex = hex::encode(&serialized);

        assert_eq!(serialized.len(), 68);
        assert_eq!(hex, "43410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac");
        Ok(())
    }
}


