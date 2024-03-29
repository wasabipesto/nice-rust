//! A client for distributed search of square-cube pandigitals
//!
//! This script connects to my server running the nice-backend at https://nicenumbers.net.
//! The API structure is described in detail at https://github.com/wasabipesto/nice-backend-v.

use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;
use std::time::Instant;

extern crate rayon;
use rayon::prelude::*;

extern crate malachite;
use malachite::natural::Natural;
use malachite::num::arithmetic::traits::{CeilingRoot, DivAssignRem, FloorRoot, Mod, Pow};
use malachite::num::basic::traits::{One, Zero};
use malachite::num::conversion::traits::Digits;

extern crate reqwest;
extern crate serde;
use serde::{Deserialize, Serialize};

extern crate clap;
use clap::ValueEnum; // have to derive enum for cli

const CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const MAX_SUPPORTED_BASE_NORMAL: u32 = 97;
const MAX_SUPPORTED_BASE_HIGH: u32 = 120;
const NEAR_MISS_CUTOFF_PERCENT: f32 = 0.9;
const BENCHMARK_DEFAULT_BASE: u32 = 40;
const BUCNHMARK_DEFAULT_RANGE: u32 = 100000;

mod api_common;
use api_common::{
    deserialize_string_to_natural, get_field_benchmark, get_field_from_server,
    submit_field_to_server,
};

mod process_integer;
mod process_natural;

mod residue_filter;
use self::residue_filter::get_residue_filter;

mod base_range;
use self::base_range::get_base_range;

/// Each possible search mode the server and client supports.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    Detailed,
    Niceonly,
}

/// A field returned from the server. Used as input for processing.
#[derive(Debug, Deserialize, Clone)]
pub struct FieldClaim {
    pub id: u32,
    pub username: String,
    pub base: u32,
    #[serde(deserialize_with = "deserialize_string_to_natural")]
    pub search_start: Natural,
    #[serde(deserialize_with = "deserialize_string_to_natural")]
    pub search_end: Natural,
    #[serde(deserialize_with = "deserialize_string_to_natural")]
    pub search_range: Natural,
}

/// The compiled results sent to the server after processing. Options for both modes.
#[derive(Debug, Serialize, PartialEq)]
pub struct FieldSubmit {
    pub id: u32,
    pub username: String,
    pub client_version: String,
    pub unique_count: Option<HashMap<u32, u32>>,
    pub near_misses: Option<HashMap<String, u32>>,
    pub nice_list: Option<Vec<String>>,
}

/// Run the program following the specified flow.
pub fn run(
    mode: Mode,
    api_base: String,
    username: String,
    quiet: bool,
    verbose: bool,
    benchmark: bool,
    parallel: bool,
    high_bases: bool,
    base: Option<u32>,
    range: Option<u32>,
    field: Option<u32>,
) {
    let claim_data = if benchmark {
        get_field_benchmark(base, range)
    } else {
        get_field_from_server(
            &mode,
            &high_bases,
            &api_base,
            &username,
            &base,
            &range,
            &field,
        )
    };
    if !quiet {
        println!("{:?}", claim_data);
    }
    let before = Instant::now();

    // process range & compile results
    let submit_data: FieldSubmit = match high_bases {
        false => match mode {
            Mode::Detailed => process_integer::process_detailed(&claim_data, parallel),
            Mode::Niceonly => process_integer::process_niceonly(&claim_data, parallel),
        },
        true => match mode {
            Mode::Detailed => process_natural::process_detailed(&claim_data),
            Mode::Niceonly => process_natural::process_niceonly(&claim_data),
        },
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
        submit_field_to_server(&mode, &api_base, submit_data)
    }
}
