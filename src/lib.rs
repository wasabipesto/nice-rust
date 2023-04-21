//! A client for distributed search of square-cube pandigitals
//!
//! This script connects to my server running the nice-backend at https://nicenumbers.net.
//! The API structure is described in detail at https://github.com/wasabipesto/nice-backend-v.

use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;
use std::time::Instant;

extern crate malachite;
use malachite::natural::Natural;
use malachite::num::arithmetic::traits::{CeilingRoot, DivAssignRem, FloorRoot, Mod, Pow};
use malachite::num::basic::traits::{One, Zero};

extern crate reqwest;
extern crate serde;
use serde::{Deserialize, Serialize};

extern crate clap;
use clap::ValueEnum; // have to derive enum for cli

const CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const MAX_SUPPORTED_BASE: u32 = 120;
const MAX_SUPPORTED_RANGE: u32 = u32::MAX;
const NEAR_MISS_CUTOFF_PERCENT: f32 = 0.9;

mod api_com;
use api_com::{get_field, get_field_benchmark, submit_field, FieldSubmit};

mod nice_com;
use nice_com::{process_detailed_natural, process_niceonly_natural};

/// Each possible search mode the server and client supports.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    Detailed,
    Niceonly,
}

/// Run the program following the specified flow.
pub fn run(
    mode: Mode,
    api_base: String,
    username: String,
    quiet: bool,
    verbose: bool,
    benchmark: bool,
    base: Option<u32>,
    max_range: Option<u128>,
    field: Option<u128>,
) {
    let claim_data = if benchmark {
        get_field_benchmark(max_range)
    } else {
        get_field(&mode, &api_base, &username, &base, &max_range, &field)
    };
    if !quiet {
        println!("{:?}", claim_data);
    }
    let before = Instant::now();

    // process range & compile results
    let submit_data: FieldSubmit = match mode {
        Mode::Detailed => process_detailed_natural(&claim_data),
        Mode::Niceonly => process_niceonly_natural(&claim_data),
    };

    if !quiet {
        println!("{:?}", submit_data);
    }
    if benchmark || verbose {
        println!("Elapsed time: {:.3?}", before.elapsed());
        println!(
            "Hash rate:    {:.3e}",
            f64::try_from(&claim_data.search_range).unwrap() / before.elapsed().as_secs_f64()
        );
    }
    if !benchmark {
        submit_field(&mode, &api_base, submit_data)
    }
}
