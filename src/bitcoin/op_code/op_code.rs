use crate::algorithms::{hash160::hash160, hash256::hash256};

pub enum OP_CODE_FUNCTIONS {
    Op0, // 0x00
    Op1, // 0x51
    Op2, // 0x52
    Op3, // 0x53
    Op4, // 0x54
    Op5, // 0x55
    Op6, // 0x56
    Op7, // 0x57
    Op8, // 0x58
    Op9, // 0x59
    Op10, // 0x5a
    Op11, // 0x5b
    Op12, // 0x5c
    Op13, // 0x5d
    Op14, // 0x5e
    Op15, // 0x5f
    OpDup, //0x76 118
    OpAdd, // 0x93
    OpHash256, // 0xa9 170
    OpCheckSig, // 0xac 172
}

pub fn op_dup(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 1 {
        return false;
    }
    stack.push(stack.last().unwrap().clone());
    true
}

pub fn op_hash256(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 1 {
        return false;
    }
    let el = stack.pop().unwrap();
    let hash = hash256(&el).to_vec();
    stack.push(hash);
    true
}

pub fn op_hash160(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 1 {
        return false;
    }
    let el = stack.pop().unwrap();
    let hash = hash160(&el).to_vec();
    stack.push(hash);
    true
}