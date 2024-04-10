use std::time::SystemTime;
use chrono::{DateTime, Utc};
use primitive_io::{Inputs, Outputs};

fn print_current_time(tag: &str) {
    let current_time = SystemTime::now();
    let current_time = DateTime::<Utc>::from(current_time);
    let current_time_str = current_time.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    println!{"{} at {}", tag, current_time_str};
}

pub fn main() {
    let (prove_dcap_verify, verify_dcap_verify) = guest::build_dcap_verify();

    let now = 1699301000u64;
    let quote = include_bytes!("../res/dcap_quote").to_vec();
    let quote_collateral = include_bytes!("../res/dcap_quote_collateral").to_vec();

    let input = Inputs {
        now,
        quote,
        quote_collateral,
    };

    print_current_time("prove start");
    let (output, proof) = prove_dcap_verify(input);
    print_current_time("prove finished");
    println!("proof size: {:?}", proof.size());

    // let is_valid = verify_dcap_verify(proof);

    // println!("output: {}", output);
    // println!("valid: {}", is_valid);
}
