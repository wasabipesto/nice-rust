use std::env;
use std::collections::HashMap;

extern crate clap;
use clap::Parser;

extern crate malachite;
use malachite::{Natural, num::conversion::traits::Digits};
use malachite::num::arithmetic::traits::Pow;

use std::time::Instant;

extern crate reqwest;
extern crate serde;
use serde::{Serialize, Deserialize};

const CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(
        short, 
        long, 
        default_value="anonymous",
        help="the username to send alongside your contribution"
    )]
    username: String,

    #[arg(
        long,
        help="run an offline benchmark"
    )]
    benchmark: bool,

    #[arg(
        short,
        long,
        help="suppress some output"
    )]
    quiet: bool,
}

#[derive(Debug, Deserialize)]
struct FieldClaim {
    search_id: u32,
    base: u32,
    search_start: u128, // u128 will only get us to base 97
    search_end: u128,
    //claimed_time: String,
    //claimed_by: String,
    //expiration_time: String
}

#[derive(Debug, Serialize)]
struct FieldSubmit<'me> {
    search_id: u32,
    username: &'me str,
    client_version: &'static str,
    unique_count: HashMap<u32,u32>,
    near_misses: HashMap<u128,u32>
}

// get a static field for benchmarking
// TODO: add additional benchmark ranges
fn get_field_benchmark() -> FieldClaim {
    return FieldClaim {
        search_id: 15,
        base: 28,
        search_start: 52260814,
        search_end: 91068707,
    };
}

// get a field from the server - detailed
fn get_field_detailed(username: &str) -> FieldClaim {
    let query_url = "https://nice.wasabipesto.com/claim?username=".to_owned() + username;
    let claim_data: Result<FieldClaim, reqwest::Error> = reqwest::blocking::get(query_url)
        .unwrap().json();
    claim_data.unwrap()
}

// submit field data to the server - detailed
fn submit_field_detailed(submit_data: FieldSubmit) {
    let client = reqwest::blocking::Client::new();
    let _response = client.post("https://nice.wasabipesto.com/submit")
        .json(&submit_data)
        .send();
}

// get the number of unique digits in the concatenated sqube of a specified number
fn get_num_uniques(num: Natural, base: u32, digits_indicator: &mut Vec<bool>) -> u32 {

    // clear the array
    digits_indicator.fill(false);

    // square the number
    let squared = (&num).pow(2);

    // convert to base & save the digits in the array
    for digit in squared.to_digits_asc(&base) { 
        digits_indicator[digit as usize] = true;
    }

    // cube the number
    let cubed = squared * num;

    // convert to base & save the digits in the array
    for digit in cubed.to_digits_asc(&base) { 
        digits_indicator[digit as usize] = true;
    }

    // output the number of unique digits
    let mut unique_digits = 0;
    for digit in digits_indicator {
        if *digit {unique_digits += 1}
    }
    return unique_digits
}

#[test]
fn test_get_num_uniques() {
    assert_eq!(
        get_num_uniques(Natural::from(69 as u128), 10, &mut vec![false; 10 as usize]), 
        10
    );
    assert_eq!(
        get_num_uniques(Natural::from(256 as u128), 2, &mut vec![false; 2 as usize]), 
        2
    );
    assert_eq!(
        get_num_uniques(Natural::from(123 as u128), 8, &mut vec![false; 8 as usize]), 
        8
    );
    assert_eq!(
        get_num_uniques(Natural::from(15 as u128), 16, &mut vec![false; 16 as usize]), 
        5
    );
    assert_eq!(
        get_num_uniques(Natural::from(100 as u128), 99, &mut vec![false; 99 as usize]), 
        3
    );
    assert_eq!(
        get_num_uniques(Natural::from(4134931983708 as u128), 40, &mut vec![false; 40 as usize]), 
        39
    );
    assert_eq!(
        get_num_uniques(Natural::from(173583337834150 as u128), 44, &mut vec![false; 44 as usize]), 
        41
    );
}

// get detailed niceness data on a range of numbers and aggregate it
fn process_range_detailed(n_start: u128, n_end: u128, base: u32) -> (Vec<u128>,HashMap<u32,u32>) {

    // near_misses_cutoff: minimum number of uniques required for the nbumber to be recorded
    let near_misses_cutoff: u32 = (base as f32 * 0.9) as u32;

    // near_misses: list of numbers with niceness ratio (uniques/base) above the cutoff
    // pre-allocate memory for the maximum possible number of near misses (wastes memory but saves resizing)
    let mut near_misses: Vec<u128> = Vec::with_capacity((n_end - n_start) as usize);
    
    // qty_uniques: the quantity of numbers with each possible niceness
    let mut qty_uniques: Vec<u32> = vec![0; base as usize];

    // create a boolean array that represents all possible digits
    let mut digits_indicator: Vec<bool> = vec![false; base as usize];

    // loop for all items in range (try to optimize anything in here)
    for num in n_start..n_end { 

        // get the number of uniques in the sqube
        let num_uniques: u32 = get_num_uniques(Natural::from(num), base, &mut digits_indicator);

        // check if it's nice enough to record in near_misses
        if num_uniques > near_misses_cutoff {
            near_misses.push(num);
        }

        // update our quantity distribution in qty_uniques
        qty_uniques[num_uniques as usize - 1] += 1;
    }

    // build the initial values (api expects it)
    let mut dict_qty_uniques: HashMap<u32, u32> = HashMap::new();
    for (num, count) in qty_uniques.iter().enumerate() { 
        dict_qty_uniques.insert(num as u32 + 1, *count);
    }

    // return it as a tuple
    return (near_misses, dict_qty_uniques)
}

#[test]
fn test_process_range_detailed() {
    assert_eq!(
        process_range_detailed(47, 100, 10),
        (
            Vec::from([
                69,
            ]),
            HashMap::from([
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 4),
                (5, 5),
                (6, 15),
                (7, 20),
                (8, 7),
                (9, 1),
                (10, 1),
            ])
        )
    );
    assert_eq!(
        process_range_detailed(144, 329, 12),
        (
            Vec::from([]),
            HashMap::from([
                (1, 0),
                (2, 1),
                (3, 1),
                (4, 6),
                (5, 15),
                (6, 27),
                (7, 55),
                (8, 53),
                (9, 24),
                (10, 3),
                (11, 0),
                (12, 0),
            ])
        )
    );
}

fn main() {

    // parse args from command line
    let cli = Cli::parse();

    // get the field to search
    let claim_data = if cli.benchmark { get_field_benchmark() } else { get_field_detailed(&cli.username) };

    // print debug information
    if ! cli.quiet { println!("{:?}", claim_data); }

    // start a timer
    let before = Instant::now();

    // search for near_misses and qty_uniques
    let (
        near_misses, 
        qty_uniques
    ) = process_range_detailed(
        claim_data.search_start,
        claim_data.search_end,
        claim_data.base,
    );

    // debug: print the timer
    if cli.benchmark { println!("Elapsed time: {:.4?}", before.elapsed()); }

    // convert the near_misses list into a map of {num, uniques}
    let mut near_miss_map: HashMap<u128,u32> = HashMap::new();
    for nm in near_misses.iter() {
        near_miss_map.insert(
            *nm,
            get_num_uniques(
                Natural::from(*nm),
                claim_data.base,
                &mut vec![false; claim_data.base as usize]
            )
        );
    }

    // compile results
    let submit_data = FieldSubmit { 
        search_id: claim_data.search_id,
        username: &cli.username,
        client_version: &CLIENT_VERSION,
        unique_count: qty_uniques,
        near_misses: near_miss_map
    };
    // print debug information
    if ! cli.quiet { println!("{:?}", submit_data); }
    
    // upload results (only if not doing benchmarking)
    if ! cli.benchmark { submit_field_detailed(submit_data) }
}