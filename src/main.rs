use std::env;
use std::collections::HashMap;

//extern crate num_bigint;
//use num_bigint::BigUint;

extern crate reqwest;

extern crate serde;
use serde::{Serialize, Deserialize};

const CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Deserialize)]
struct FieldClaim {
    search_id: u32,
    base: u32,
    search_start: u128,
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

// represent a number in a specified base
// returns a list of digits from MSD to LSD
fn number_to_base(num: u128, b: u128) -> Vec<u128> {
    let mut n = num;
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % b);
        n /= b;
    }
    digits.reverse();
    return digits;
}

#[test]
fn test_number_to_base() {
    assert_eq!(
        number_to_base(256, 2), 
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        number_to_base(123, 8), 
        vec![1, 7, 3]
    );
    assert_eq!(
        number_to_base(15,16), 
        vec![15]
    );
    assert_eq!(
        number_to_base(100, 99), 
        vec![1, 1]
    );
}

// get the number of unique digits in the concatenated sqube of a specified number
fn get_num_uniques(num: u128, base: u32) -> u32 {
    let b = base as u128;
    let mut sqube = number_to_base(num.pow(2), b);
    sqube.append(&mut number_to_base(num.pow(3), b));
    sqube.sort();
    sqube.dedup();
    return sqube.len() as u32;
}

#[test]
fn test_get_num_uniques() {
    assert_eq!(
        get_num_uniques(69, 10), 
        10
    );
    assert_eq!(
        get_num_uniques(256, 2), 
        2
    );
    assert_eq!(
        get_num_uniques(123, 8), 
        8
    );
    assert_eq!(
        get_num_uniques(15, 16), 
        5
    );
    assert_eq!(
        get_num_uniques(100, 99), 
        3
    );
}

// get niceness data on a range of numbers and aggregate it
fn search_range(n_start: u128, n_end: u128, base: u32) -> (Vec<u128>,HashMap<u32,u32>) {
    let near_misses_cutoff = base as f32 * 0.9; // minimum number of uniques to be counted
    let mut near_misses: Vec<u128> = Vec::new(); // numbers with niceness (uniques/base) >= 0.9
    
    let mut qty_uniques = HashMap::new(); // the quantity of numbers with each possible niceness
    for b in 1..base+1 { // build dict initial values
        qty_uniques.insert(b,0);
    }

    for num in n_start..n_end { // loop for all items in range
        let num_uniques: u32 = get_num_uniques(num, base);
        if num_uniques as f32 > near_misses_cutoff { // check niceness
            near_misses.push(num); // pretty nice, push to near_misses
        }
        qty_uniques.insert( // update qty_uniques distribution
            num_uniques, 
            qty_uniques.get(&num_uniques).copied().unwrap_or(0)+1
        );
    }
    return (near_misses,qty_uniques)
}

// get the claim data from the server
fn get_claim_data(username: &str) -> FieldClaim {
    let query_url = "https://nice.wasabipesto.com/claim?username=".to_owned() + &username;
    let claim_data: Result<FieldClaim, reqwest::Error> = reqwest::blocking::get(query_url).unwrap().json();
    return claim_data.unwrap();
}

fn main() {
    // get username from first argument
    let mut args = env::args();
    let username = args.by_ref().skip(1).next().unwrap_or_else(|| {
        "anonymous".to_string()
    });

    // get search data
    let claim_data = get_claim_data(&username);
    println!("{:?}", claim_data);

    // search for near_misses and qty_uniques
    let (
        near_misses, 
        qty_uniques
    ) = search_range(
        claim_data.search_start,
        claim_data.search_end,
        claim_data.base,
    );
    
    let mut near_miss_map: HashMap<u128,u32> = HashMap::new();
    for nm in near_misses.iter() {
        near_miss_map.insert(
            *nm,
            get_num_uniques(
                *nm,
                claim_data.base
            )
        );
    }

    // compile results
    let submit_data = FieldSubmit { 
        search_id: claim_data.search_id,
        username: &username,
        client_version: &CLIENT_VERSION,
        unique_count: qty_uniques,
        near_misses: near_miss_map
    };
    println!("{:?}", submit_data);
    
    // upload results
    let client = reqwest::blocking::Client::new();
    let _response = client.post("https://nice.wasabipesto.com/submit")
        .json(&submit_data)
        .send();

    // show response (debug)
    //println!("{:?}", response);
}