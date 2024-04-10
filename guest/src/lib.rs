#![cfg_attr(feature = "guest", no_std)]
#![no_main]

extern crate alloc;

use scale_codec::Decode;

mod error;
use error::Error;
mod dcap;

use primitive_io::{Inputs, Outputs};

#[jolt::provable]
fn dcap_verify(input: &Inputs) -> Outputs {
    let now = input.now;
    let raw_quote = &input.quote;
    let raw_quote_collateral = &input.quote_collateral;

    let quote_collateral =
        dcap::SgxV30QuoteCollateral::decode(&mut raw_quote_collateral.as_slice()).unwrap();
    let (report_data, mr_enclave, mr_signer, isv_prod_id, isv_svn, tcb_status, advisory_ids) =
        dcap::verify(&raw_quote, &quote_collateral, now).unwrap();

    let output = Outputs {
        report_data,
        mr_enclave,
        mr_signer,
        isv_prod_id,
        isv_svn,
        tcb_status,
        advisory_ids
    };

    output
}
