use std::io::{self, Read};
use crypto_bigint::U256;

use crate::{algorithms::endian::int_to_little_endian, bitcoin::{tx::{tx_in::TxIn, tx_out::TxOut}, utils::read_u32_le, varint::{encode_varint, read_varint}}};

pub enum Network {
    MAINNET,
    TESTNET
}

pub struct Tx {
    pub version: u32,
    pub tx_ins: Vec<TxIn>,
    pub tx_outs: Vec<TxOut>,
    pub locktime: u32,
    pub network: Network
}

impl Tx {
    pub fn new(version: u32, tx_ins: Vec<TxIn>, tx_outs: Vec<TxOut>, locktime: u32, network: Network) -> Self {
        Self { version, tx_ins, tx_outs, locktime, network }
    }

    pub fn id(self) -> String {
        String::from("")
    }

    /*
    pub fn hash(self) -> &[u8] {
        &[]
    }
    */

    pub fn parse<R: Read>(stream: &mut R) -> io::Result<Tx> {
        let version = read_u32_le(stream)?;

        let tx_in_count = read_varint(stream)?;
        let mut tx_ins: Vec<TxIn> = Vec::new();
        for _i in 0..tx_in_count {
            tx_ins.push(TxIn::parse(stream)?);
        }

        let tx_out_count = read_varint(stream)?;
        let mut tx_outs: Vec<TxOut> = Vec::new();
        for _i in 0..tx_out_count {
            tx_outs.push(TxOut::parse(stream)?);
        }

        let locktime = read_u32_le(stream)?;

        Ok(Tx::new(version, tx_ins, tx_outs, locktime, Network::MAINNET))
    }

    pub fn serialize(self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&int_to_little_endian(U256::from_u32(self.version), 4));

        let ins_len = self.tx_ins.len() as u64;
        let ins_len_varint = encode_varint(U256::from_u64(ins_len));
        result.extend_from_slice(&ins_len_varint);

        for tx_in in self.tx_ins {
            result.extend_from_slice(&tx_in.serialize());
        }

        let outs_len = self.tx_outs.len() as u64;
        let outs_len_varint = encode_varint(U256::from_u64(outs_len));
        result.extend_from_slice(&outs_len_varint);

        for tx_out in self.tx_outs {
            result.extend_from_slice(&tx_out.serialize());
        }

        result.extend_from_slice(&int_to_little_endian(U256::from_u32(self.locktime), 4));

        result
    }

}

#[cfg(test)]
mod tx_tests {
    use std::io::Cursor;
    use super::*;

    /*
        01000000 
        04 
          - 56919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e 01000000
            6a 47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7
            feffffff
          - eb8f51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3 0000000
            6a 47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937
            feffffff
          - 567bf40595119d1bb8a3037c356efd56170b64cbcc160fb028fa10704b45d775 0000000
            6a 47304402204c7c7818424c7f7911da6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5c
            feffffff
          - d63a5e6c16e620f86f375925b21cabaf736c779f88fd04dcad51d26690f7f345 0000000
            6a 47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20dfe7520d55
            feffffff
        02
          - 51430f0000000000
            19 76a914ab0c0b2e98b1ab6dbf67d4750b0a56244948a87988ac
          - 005a620200000000
            19 76a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac
        46430600
        Version: 01000000 (1)
        TxIn: 04 (4)
            0: Outpoint: 56919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e:1
            0: ScriptSig: 47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7
            0: Sequence: feffffff
            1: Outpoint: eb8f51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3:0
            1: ScriptSig: 47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937
            1: Sequence: feffffff
            2: Outpoint: 567bf40595119d1bb8a3037c356efd56170b64cbcc160fb028fa10704b45d775:0
            2: ScriptSig: 47304402204c7c7818424c7f7911da6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5c
            2: Sequence: feffffff
            3: Outpoint: d63a5e6c16e620f86f375925b21cabaf736c779f88fd04dcad51d26690f7f345:1
            3: ScriptSig: 47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20dfe7520d55
            3: Sequence: feffffff
        TxOut: 02 (2)
            0: Sats: 51430f0000000000 (1,000,273)
            0: ScriptPubKey: 76a914ab0c0b2e98b1ab6dbf67d4750b0a56244948a87988ac
            1: Sats: 005a620200000000 (40,000,000)
            1: ScriptPubKey: 76a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac
        Locktime: 46430600 (410438)
    */
    #[test]
    fn parse_test() -> Result<(), Box<dyn std::error::Error>> {
        let tx_bytes = hex::decode("010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e010000006a47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7feffffffeb8f51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3000000006a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937feffffff567bf40595119d1bb8a3037c356efd56170b64cbcc160fb028fa10704b45d775000000006a47304402204c7c7818424c7f7911da6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5cfeffffffd63a5e6c16e620f86f375925b21cabaf736c779f88fd04dcad51d26690f7f345010000006a47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20dfe7520d55feffffff0251430f00000000001976a914ab0c0b2e98b1ab6dbf67d4750b0a56244948a87988ac005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac46430600")?;
        let mut stream = Cursor::new(tx_bytes);
        let tx = Tx::parse(&mut stream)?;
        assert_eq!(tx.version, 1);

        assert_eq!(tx.tx_ins.len(), 4);

        let tx1_id_vec = hex::decode("56919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e")?;
        let tx1_id: [u8; 32] = tx1_id_vec.try_into().unwrap();
        let tx1_sig = hex::decode("47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7")?;
        let tx1_sequence_vec = hex::decode("feffffff")?;
        let tx1_sequence_u8: [u8; 4] = tx1_sequence_vec.try_into().unwrap();
        let tx1_sequence = u32::from_le_bytes(tx1_sequence_u8);
        assert_eq!(tx.tx_ins[0].previous_output.tx_id, tx1_id); 
        assert_eq!(tx.tx_ins[0].previous_output.vout, 1); 
        assert_eq!(tx.tx_ins[0].script_sig, tx1_sig); 
        assert_eq!(tx.tx_ins[0].sequence, tx1_sequence); 

        let tx2_id_vec = hex::decode("eb8f51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3")?;
        let tx2_id: [u8; 32] = tx2_id_vec.try_into().unwrap();
        let tx2_sig = hex::decode("47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937")?;
        let tx2_sequence_vec = hex::decode("feffffff")?;
        let tx2_sequence_u8: [u8; 4] = tx2_sequence_vec.try_into().unwrap();
        let tx2_sequence = u32::from_le_bytes(tx2_sequence_u8);
        assert_eq!(tx.tx_ins[1].previous_output.tx_id, tx2_id); 
        assert_eq!(tx.tx_ins[1].previous_output.vout, 0); 
        assert_eq!(tx.tx_ins[1].script_sig, tx2_sig); 
        assert_eq!(tx.tx_ins[1].sequence, tx2_sequence); 

        let tx3_id_vec = hex::decode("567bf40595119d1bb8a3037c356efd56170b64cbcc160fb028fa10704b45d775")?;
        let tx3_id: [u8; 32] = tx3_id_vec.try_into().unwrap();
        let tx3_sig = hex::decode("47304402204c7c7818424c7f7911da6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5c")?;
        let tx3_sequence_vec = hex::decode("feffffff")?;
        let tx3_sequence_u8: [u8; 4] = tx3_sequence_vec.try_into().unwrap();
        let tx3_sequence = u32::from_le_bytes(tx3_sequence_u8);
        assert_eq!(tx.tx_ins[2].previous_output.tx_id, tx3_id); 
        assert_eq!(tx.tx_ins[2].previous_output.vout, 0); 
        assert_eq!(tx.tx_ins[2].script_sig, tx3_sig); 
        assert_eq!(tx.tx_ins[2].sequence, tx3_sequence); 

        let tx4_id_vec = hex::decode("d63a5e6c16e620f86f375925b21cabaf736c779f88fd04dcad51d26690f7f345")?;
        let tx4_id: [u8; 32] = tx4_id_vec.try_into().unwrap();
        let tx4_sig = hex::decode("47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20dfe7520d55")?;
        let tx4_sequence_vec = hex::decode("feffffff")?;
        let tx4_sequence_u8: [u8; 4] = tx4_sequence_vec.try_into().unwrap();
        let tx4_sequence = u32::from_le_bytes(tx4_sequence_u8);
        assert_eq!(tx.tx_ins[3].previous_output.tx_id, tx4_id); 
        assert_eq!(tx.tx_ins[3].previous_output.vout, 1); 
        assert_eq!(tx.tx_ins[3].script_sig, tx4_sig); 
        assert_eq!(tx.tx_ins[3].sequence, tx4_sequence); 

        assert_eq!(tx.tx_outs.len(), 2);

        let tx1_script = hex::decode("76a914ab0c0b2e98b1ab6dbf67d4750b0a56244948a87988ac")?;
        assert_eq!(tx.tx_outs[0].amount, 1_000_273u64); 
        assert_eq!(tx.tx_outs[0].script_pubkey, tx1_script); 

        let tx2_script = hex::decode("76a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac")?;
        assert_eq!(tx.tx_outs[1].amount, 40_000_000u64); 
        assert_eq!(tx.tx_outs[1].script_pubkey, tx2_script); 

        assert_eq!(tx.locktime, 410438); 

        Ok(())
    }

    #[test]
    fn serialize_test() -> Result<(), Box<dyn std::error::Error>> {

        let sequence_vec = hex::decode("feffffff")?;
        let sequence_u8: [u8; 4] = sequence_vec.try_into().unwrap();
        let sequence = u32::from_le_bytes(sequence_u8);

        let tx_in1_id: Vec<u8> = hex::decode("56919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e")?;
        let tx_in1_id_32: [u8; 32] = tx_in1_id.try_into().unwrap();
        let tx_in1 = TxIn::new(
            tx_in1_id_32,
            1,
            hex::decode("47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7")?,
            sequence
        );

        let tx_in2_id: Vec<u8> = hex::decode("eb8f51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3")?;
        let tx_in2_id_32: [u8; 32] = tx_in2_id.try_into().unwrap();
        let tx_in2 = TxIn::new(
            tx_in2_id_32,
            0,
            hex::decode("47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937")?,
            sequence
        );

        let tx_in3_id: Vec<u8> = hex::decode("567bf40595119d1bb8a3037c356efd56170b64cbcc160fb028fa10704b45d775")?;
        let tx_in3_id_32: [u8; 32] = tx_in3_id.try_into().unwrap();
        let tx_in3 = TxIn::new(
            tx_in3_id_32,
            0,
            hex::decode("47304402204c7c7818424c7f7911da6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5c")?,
            sequence
        );

        let tx_in4_id: Vec<u8> = hex::decode("d63a5e6c16e620f86f375925b21cabaf736c779f88fd04dcad51d26690f7f345")?;
        let tx_in4_id_32: [u8; 32] = tx_in4_id.try_into().unwrap();
        let tx_in4 = TxIn::new(
            tx_in4_id_32,
            1,
            hex::decode("47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20dfe7520d55")?,
            sequence
        );

        let tx_ins = vec![tx_in1, tx_in2, tx_in3, tx_in4];

        let tx_out1 = TxOut::new(
            1_000_273u64,
            hex::decode("76a914ab0c0b2e98b1ab6dbf67d4750b0a56244948a87988ac")?,
        );

        let tx_out2 = TxOut::new(
            40_000_000u64,
            hex::decode("76a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac")?,
        );

        let tx_outs = vec![tx_out1, tx_out2];

        let tx = Tx::new(1, tx_ins, tx_outs, 410438, Network::MAINNET);

        let serialized = tx.serialize();

        let tx_bytes = hex::decode("010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e010000006a47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7feffffffeb8f51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3000000006a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937feffffff567bf40595119d1bb8a3037c356efd56170b64cbcc160fb028fa10704b45d775000000006a47304402204c7c7818424c7f7911da6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5cfeffffffd63a5e6c16e620f86f375925b21cabaf736c779f88fd04dcad51d26690f7f345010000006a47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20dfe7520d55feffffff0251430f00000000001976a914ab0c0b2e98b1ab6dbf67d4750b0a56244948a87988ac005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac46430600")?;
        assert_eq!(serialized, tx_bytes);

        Ok(())
    }
}