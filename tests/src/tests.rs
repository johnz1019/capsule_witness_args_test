use super::*;
use ckb_testtool::context::Context;
use ckb_tool::ckb_types::{
    bytes::Bytes,
    core::{Capacity, TransactionBuilder, TransactionView},
    packed::{self, *},
    prelude::*,
};
use ckb_tool::{
    ckb_error::assert_error_eq,
    ckb_hash::{blake2b_256, new_blake2b},
};
use ckb_tool::{ckb_script::ScriptError, ckb_types::core::ScriptHashType};

const MAX_CYCLES: u64 = 10_000_000;

// error numbers
const ERROR_EMPTY_ARGS: i8 = 5;

#[test]
fn test_witness_args() {
    let witness_lock_len = 1041;
    let witness = WitnessArgs::default().as_builder().build();
    let zero_lock: BytesOpt = {
        let mut buf = Vec::new();
        buf.resize(witness_lock_len, 0);
        // buf.info()
        BytesOpt::new_unchecked(buf.into())
    };
    // println!("zero_lock {:x?}", zero_lock);
    let witness_for_digest = witness.clone().as_builder().lock(zero_lock).build();
    let witness_len = witness_for_digest.as_bytes().len() as u64;
    println!(
        "witness_for_digest {:?} {:x?}",
        witness_len, witness_for_digest
    );
}
