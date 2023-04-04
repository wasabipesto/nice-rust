use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;

extern crate clap;
use clap::{Args, Parser, Subcommand};

extern crate malachite;
use malachite::natural::Natural;
use malachite::num::arithmetic::traits::{DivAssignRem, Pow};
use malachite::num::conversion::traits::Digits;

use std::time::Instant;

extern crate reqwest;
extern crate serde;
use serde::{Deserialize, Serialize};

const CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const MAX_SUPPORTED_BASE: u32 = 97;

#[cfg(test)]
mod tests;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(
        long,
        default_value = "https://nicenumbers.net/api",
        help = "the base API URL to connect to"
    )]
    api_base: String,

    #[arg(
        short,
        long,
        default_value = "anonymous",
        help = "the username to send alongside your contribution"
    )]
    username: String,

    #[arg(short, long, help = "suppress some output")]
    quiet: bool,

    #[arg(short, long, help = "show additional output")]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// perform detailed analysis
    Detailed(APIArgs),

    /// search for 100% nice numbers
    Niceonly(APIArgs),
}

#[derive(Args)]
struct APIArgs {
    #[arg(short, long, help = "request a range in a specific base")]
    base: Option<u32>,

    #[arg(short = 'r', long, help = "request a differently-sized range")]
    max_range: Option<u128>,

    #[arg(long, help = "request a specific field by id")]
    field: Option<u128>,

    #[arg(long, help = "run an offline benchmark")]
    benchmark: bool,
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

// get a static field for benchmarking
fn get_field_benchmark(max_range: &Option<u128>) -> FieldClaim {
    let search_end = match max_range {
        Some(range) => 91068707.min(52260814 + range),
        _ => 91068707,
    };
    return FieldClaim {
        id: 15,
        base: 28,
        search_start: 52260814,
        search_end: search_end,
    };
}

// get a field from the server - detailed
fn get_field_detailed(
    api_base: &str,
    username: &str,
    base: &Option<u32>,
    max_range: &Option<u128>,
    field: &Option<u128>,
) -> FieldClaim {
    let mut query_url = api_base.to_owned() + &"/claim/detailed?username=".to_owned() + username;
    if let Some(base_val) = base {
        query_url += &("&base=".to_owned() + &base_val.to_string());
    }
    if let Some(max_range_val) = max_range {
        query_url += &("&max_range=".to_owned() + &max_range_val.to_string());
    }
    if let Some(field_id_val) = field {
        query_url += &("&field=".to_owned() + &field_id_val.to_string());
    }
    let claim_data: Result<FieldClaim, reqwest::Error> =
        reqwest::blocking::get(query_url).unwrap().json();
    claim_data.unwrap()
}

// get a field from the server - nice only
fn get_field_niceonly(
    api_base: &str,
    username: &str,
    base: &Option<u32>,
    max_range: &Option<u128>,
    field: &Option<u128>,
) -> FieldClaim {
    let mut query_url = api_base.to_owned() + &"/claim/niceonly?username=".to_owned() + username;
    if let Some(base_val) = base {
        query_url += &("&base=".to_owned() + &base_val.to_string());
    }
    if let Some(max_range_val) = max_range {
        query_url += &("&max_range=".to_owned() + &max_range_val.to_string());
    }
    if let Some(field_id_val) = field {
        query_url += &("&field=".to_owned() + &field_id_val.to_string());
    }
    let claim_data: Result<FieldClaim, reqwest::Error> =
        reqwest::blocking::get(query_url).unwrap().json();
    claim_data.unwrap()
}

// submit field data to the server - detailed
fn submit_field_detailed(api_base: &str, submit_data: FieldSubmitDetailed) {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(api_base.to_owned() + &"/submit/detailed")
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
fn submit_field_niceonly(api_base: &str, submit_data: FieldSubmitNiceonly) {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(api_base.to_owned() + &"/submit/niceonly")
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

// get the number of unique digits in the sqube of a specified number
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

// test if the given number is 100% nice
fn get_is_nice(num: &Natural, base: &Natural) -> bool {
    // create a boolean array that represents all possible digits
    let mut digits_indicator = [false; MAX_SUPPORTED_BASE as usize];

    // square the number and check those digits
    let squared = (&num).pow(2);
    let mut n = squared.clone();
    while n > 0 {
        let remainder = usize::try_from(&(n.div_assign_rem(base))).unwrap();
        if digits_indicator[remainder] {
            return false;
        }
        digits_indicator[remainder] = true;
    }

    // cube the number and check those digit
    let mut n = squared * num;
    while n > 0 {
        let remainder = usize::try_from(&(n.div_assign_rem(base))).unwrap();
        if digits_indicator[remainder] {
            return false;
        }
        digits_indicator[remainder] = true;
    }
    return true;
}

// get residue classes for a base
fn get_residue_filter(base: u32) -> Vec<u32> {
    let target_residue = base * (base - 1) / 2 % (base - 1);
    (0..(base - 1))
        .filter(|num| (num.pow(2) + num.pow(3)) % (base - 1) == target_residue)
        .collect()
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

fn process_range_niceonly(n_start: u128, n_end: u128, base: u32) -> Vec<u128> {
    let base_natural = Natural::from(base);
    let residue_filter = get_residue_filter(base);
    (n_start..n_end)
        .filter(|num| residue_filter.contains(&((num % (base as u128 - 1)) as u32)))
        .filter(|num| get_is_nice(&Natural::from(*num), &base_natural))
        .collect()
}

fn main() {
    // parse args from command line
    let cli = Cli::parse();

    match &cli.command {
        Commands::Detailed(args) => {
            // get field data
            let claim_data = if args.benchmark {
                get_field_benchmark(&args.max_range)
            } else {
                get_field_detailed(
                    &cli.api_base,
                    &cli.username,
                    &args.base,
                    &args.max_range,
                    &args.field,
                )
            };
            // print debug information
            if !cli.quiet {
                println!("{:?}", claim_data);
            }
            // start timer
            let before = Instant::now();
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
                username: &cli.username,
                client_version: &CLIENT_VERSION,
                unique_count: qty_uniques,
                near_misses: near_miss_map,
            };
            // print debug information
            if !cli.quiet {
                println!("{:?}", submit_data);
            }
            if args.benchmark || cli.verbose {
                println!("Elapsed time: {:.4?}", before.elapsed());
                println!(
                    "Hash rate:    {:.3e}",
                    (claim_data.search_end - claim_data.search_start)
                        / before.elapsed().as_secs() as u128
                );
            } else {
                // upload results
                submit_field_detailed(&cli.api_base, submit_data)
            }
        }
        Commands::Niceonly(args) => {
            // get field data
            let claim_data = if args.benchmark {
                get_field_benchmark(&args.max_range)
            } else {
                get_field_niceonly(
                    &cli.api_base,
                    &cli.username,
                    &args.base,
                    &args.max_range,
                    &args.field,
                )
            };
            // print debug information
            if !cli.quiet {
                println!("{:?}", claim_data);
            }
            // start timer
            let before = Instant::now();
            // process range
            let nice_list = process_range_niceonly(
                claim_data.search_start,
                claim_data.search_end,
                claim_data.base,
            );
            // compile results
            let submit_data = FieldSubmitNiceonly {
                id: claim_data.id,
                username: &cli.username,
                client_version: &CLIENT_VERSION,
                nice_list,
            };
            // print debug information
            if !cli.quiet {
                println!("{:?}", submit_data);
            }
            if args.benchmark || cli.verbose {
                println!("Elapsed time: {:.3?}", before.elapsed());
                println!(
                    "Hash rate:    {:.3e}",
                    (claim_data.search_end - claim_data.search_start)
                        / before.elapsed().as_secs() as u128
                );
            } else {
                // upload results
                submit_field_niceonly(&cli.api_base, submit_data)
            }
        }
    }
}
