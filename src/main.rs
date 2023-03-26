use std::collections::HashMap;
use std::env;

extern crate clap;
use clap::{Args, Parser, Subcommand};

extern crate malachite;
use malachite::num::arithmetic::traits::Pow;
use malachite::{num::conversion::traits::Digits, Natural};

use std::time::Instant;

extern crate reqwest;
extern crate serde;
use serde::{Deserialize, Serialize};

const CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, help = "suppress some output")]
    quiet: bool,
}

#[derive(Subcommand)]
enum Commands {
    Detailed(APIArgs),
    Niceonly(APIArgs),
    Benchmark(BenchmarkArgs),
}

#[derive(Args)]
struct APIArgs {
    #[arg(
        long,
        default_value = "https://nicenumbers.net/api",
        help = "the base API URL to connect to"
    )]
    api_url: String,

    #[arg(
        short,
        long,
        default_value = "anonymous",
        help = "the username to send alongside your contribution"
    )]
    username: String,

    #[arg(short, long, help = "request a range in a specific base")]
    base: Option<u32>,

    #[arg(short = 'r', long, help = "request a differently-sized range")]
    max_range: Option<u128>,
}

#[derive(Args)]
struct BenchmarkArgs {
    #[arg(short = 'r', long, help = "request a differently-sized range")]
    max_range: Option<u128>,
}

#[derive(Debug, Deserialize)]
struct FieldClaim {
    id: u32,
    base: u32,
    #[serde(deserialize_with = "deserialize_stringified_number")]
    search_start: u128,
    #[serde(deserialize_with = "deserialize_stringified_number")]
    search_end: u128,
}

#[derive(Debug, Serialize)]
struct FieldSubmitDetailed<'me> {
    id: u32,
    username: &'me str,
    client_version: &'static str,
    unique_count: HashMap<u32, u32>,
    near_misses: HashMap<u128, u32>,
}

#[derive(Debug, Serialize)]
struct FieldSubmitNiceonly<'me> {
    id: u32,
    username: &'me str,
    client_version: &'static str,
    nice_list: Vec<u128>,
}

// get a static field for benchmarking
// TODO: add additional benchmark ranges
fn get_field_benchmark() -> FieldClaim {
    return FieldClaim {
        id: 15,
        base: 28,
        search_start: 52260814,
        search_end: 91068707,
    };
}

// custom deserialization for stringified bigints
// TODO: deserialize into naturals directly
fn deserialize_stringified_number<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let s = s.trim_matches('"');
    if let Ok(number) = s.parse() {
        Ok(number)
    } else {
        Err(serde::de::Error::custom(format!("invalid number: {}", s)))
    }
}

// get a field from the server - detailed
fn get_field_detailed(
    api_url: &str,
    username: &str,
    base: &Option<u32>,
    max_range: &Option<u128>,
) -> FieldClaim {
    let mut query_url = api_url.to_owned() + &"/claim/detailed?username=".to_owned() + username;
    if let Some(base_val) = base {
        query_url += &("&base=".to_owned() + &base_val.to_string());
    }
    if let Some(max_range_val) = max_range {
        query_url += &("&max_range=".to_owned() + &max_range_val.to_string());
    }
    let claim_data: Result<FieldClaim, reqwest::Error> =
        reqwest::blocking::get(query_url).unwrap().json();
    claim_data.unwrap()
}

// get a field from the server - nice only
fn get_field_niceonly(
    api_url: &str,
    username: &str,
    base: &Option<u32>,
    max_range: &Option<u128>,
) -> FieldClaim {
    let mut query_url = api_url.to_owned() + &"/claim/niceonly?username=".to_owned() + username;
    if let Some(base_val) = base {
        query_url += &("&base=".to_owned() + &base_val.to_string());
    }
    if let Some(max_range_val) = max_range {
        query_url += &("&max_range=".to_owned() + &max_range_val.to_string());
    }
    let claim_data: Result<FieldClaim, reqwest::Error> =
        reqwest::blocking::get(query_url).unwrap().json();
    claim_data.unwrap()
}

// submit field data to the server - detailed
fn submit_field_detailed(api_url: &str, submit_data: FieldSubmitDetailed) {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(api_url.to_owned() + &"/submit/detailed")
        .json(&submit_data)
        .send();

    match response {
        Ok(res) => {
            if res.status().is_success() {
                // The request was successful, no need to handle the response body.
                return;
            }
            match res.text() {
                Ok(msg) => println!("Server returned an error: {}", msg),
                Err(_) => println!("Server returned an error."),
            }
        }
        Err(e) => {
            // Handle network errors.
            println!("Network error: {}", e);
        }
    }
}

// submit field data to the server - nice only
fn submit_field_niceonly(api_url: &str, submit_data: FieldSubmitNiceonly) {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(api_url.to_owned() + &"/submit/niceonly")
        .json(&submit_data)
        .send();

    match response {
        Ok(res) => {
            if res.status().is_success() {
                // The request was successful, no need to handle the response body.
                return;
            }
            match res.text() {
                Ok(msg) => println!("Server returned an error: {}", msg),
                Err(_) => println!("Server returned an error."),
            }
        }
        Err(e) => {
            // Handle network errors.
            println!("Network error: {}", e);
        }
    }
}

// get the number of unique digits in the concatenated sqube of a specified number
fn get_num_uniques(num: Natural, base: u32) -> u32 {
    // create a boolean array that represents all possible digits
    let mut digits_indicator: Vec<bool> = vec![false; base as usize];

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
        if digit {
            unique_digits += 1
        }
    }
    return unique_digits;
}

#[test]
fn test_get_num_uniques() {
    assert_eq!(get_num_uniques(Natural::from(69 as u128), 10), 10);
    assert_eq!(get_num_uniques(Natural::from(256 as u128), 2), 2);
    assert_eq!(get_num_uniques(Natural::from(123 as u128), 8), 8);
    assert_eq!(get_num_uniques(Natural::from(15 as u128), 16), 5);
    assert_eq!(get_num_uniques(Natural::from(100 as u128), 99), 3);
    assert_eq!(
        get_num_uniques(Natural::from(4134931983708 as u128), 40),
        39
    );
    assert_eq!(
        get_num_uniques(Natural::from(173583337834150 as u128), 44),
        41
    );
}

// get detailed niceness data on a range of numbers and aggregate it
fn process_range_detailed(n_start: u128, n_end: u128, base: u32) -> (Vec<u128>, HashMap<u32, u32>) {
    // near_misses_cutoff: minimum number of uniques required for the number to be recorded
    let near_misses_cutoff: u32 = (base as f32 * 0.9) as u32;

    // near_misses: list of numbers with niceness ratio (uniques/base) above the cutoff
    // pre-allocate memory for the maximum possible number of near misses (wastes memory but saves resizing)
    let mut near_misses: Vec<u128> = Vec::with_capacity((n_end - n_start) as usize);

    // qty_uniques: the quantity of numbers with each possible niceness
    let mut qty_uniques: Vec<u32> = vec![0; base as usize];

    // loop for all items in range (try to optimize anything in here)
    for num in n_start..n_end {
        // get the number of uniques in the sqube
        let num_uniques: u32 = get_num_uniques(Natural::from(num), base);

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
    return (near_misses, dict_qty_uniques);
}

#[test]
fn test_process_range_detailed() {
    assert_eq!(
        process_range_detailed(47, 100, 10),
        (
            Vec::from([69,]),
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

fn process_range_niceonly(n_start: u128, n_end: u128, base: u32) -> Vec<u128> {
    // nice_list: list of numbers with niceness ratio (uniques/base) above the cutoff
    let mut nice_list: Vec<u128> = vec![];

    // loop for all items in range (try to optimize anything in here)
    for num in n_start..n_end {
        // get the number of uniques in the sqube
        let num_uniques: u32 = get_num_uniques(Natural::from(num), base);

        // check if it's 100% nice
        if num_uniques == base {
            nice_list.push(num);
        }
    }

    // return the list
    return nice_list;
}

fn main() {
    // parse args from command line
    let cli = Cli::parse();

    match &cli.command {
        Commands::Detailed(args) => {
            // get field data
            let claim_data =
                get_field_detailed(&args.api_url, &args.username, &args.base, &args.max_range);
            // print debug information
            if !cli.quiet {
                println!("{:?}", claim_data);
            }
            // process range
            let (near_misses, qty_uniques) = process_range_detailed(
                claim_data.search_start,
                claim_data.search_end,
                claim_data.base,
            );
            // compile near_misses
            let mut near_miss_map: HashMap<u128, u32> = HashMap::new();
            for nm in near_misses.iter() {
                near_miss_map.insert(*nm, get_num_uniques(Natural::from(*nm), claim_data.base));
            }
            // compile results
            let submit_data = FieldSubmitDetailed {
                id: claim_data.id,
                username: &args.username,
                client_version: &CLIENT_VERSION,
                unique_count: qty_uniques,
                near_misses: near_miss_map,
            };
            // print debug information
            if !cli.quiet {
                println!("{:?}", submit_data);
            }
            // upload results
            submit_field_detailed(&args.api_url, submit_data)
        }
        Commands::Niceonly(args) => {
            // get field data
            let claim_data =
                get_field_niceonly(&args.api_url, &args.username, &args.base, &args.max_range);
            // print debug information
            if !cli.quiet {
                println!("{:?}", claim_data);
            }
            // process range
            let nice_list = process_range_niceonly(
                claim_data.search_start,
                claim_data.search_end,
                claim_data.base,
            );
            // compile results
            let submit_data = FieldSubmitNiceonly {
                id: claim_data.id,
                username: &args.username,
                client_version: &CLIENT_VERSION,
                nice_list: nice_list,
            };
            // print debug information
            if !cli.quiet {
                println!("{:?}", submit_data);
            }
            // upload results
            submit_field_niceonly(&args.api_url, submit_data)
        }
        Commands::Benchmark(_args) => {
            // get field data
            let claim_data = get_field_benchmark();
            // print debug information
            if !cli.quiet {
                println!("{:?}", claim_data);
            }
            let before = Instant::now();
            process_range_detailed(
                claim_data.search_start,
                claim_data.search_end,
                claim_data.base,
            );
            // print debug information
            if !cli.quiet {
                println!("{:?}", claim_data);
            }
            println!("Elapsed time: {:.4?}", before.elapsed());
        }
    }
}
