//! A client for distributed search of square-cube pandigitals
//!
//! This script connects to my server running the nice-backend at https://nicenumbers.net.
//! The API structure is described in detail at https://github.com/wasabipesto/nice-backend-v.

use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;
use std::time::Instant;

extern crate malachite;
use self::malachite::natural::Natural;
use self::malachite::num::arithmetic::traits::{DivAssignRem, Pow};
use self::malachite::num::conversion::traits::Digits;

extern crate reqwest;
extern crate serde;
use self::serde::{Deserialize, Serialize};

extern crate clap;
use clap::ValueEnum; // have to derive enum for cli

const CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const MAX_SUPPORTED_BASE: u32 = 97;

/// Each possible search mode the server and client supports.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    Detailed,
    Niceonly,
}

/// A field returned from the server. Used as input for processing.
#[derive(Debug, Deserialize)]
pub struct FieldClaim {
    pub id: u32,
    pub base: u32,
    #[serde(deserialize_with = "deserialize_stringified_number")]
    pub search_start: u128,
    #[serde(deserialize_with = "deserialize_stringified_number")]
    pub search_end: u128,
}

/// The compiled results sent to the server after processing. Options for both modes.
#[derive(Debug, Serialize)]
pub struct FieldSubmit<'me> {
    pub id: u32,
    pub username: &'me str,
    pub client_version: &'static str,
    pub unique_count: Option<HashMap<u32, u32>>,
    pub near_misses: Option<HashMap<u128, u32>>,
    pub nice_list: Option<Vec<u128>>,
}

/// Deserialize BigInts from the server that are wrapped in quotes.
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

/// Generate a field offline for benchmark testing.
pub fn get_field_benchmark(max_range: Option<u128>) -> FieldClaim {
    return FieldClaim {
        id: 0,
        base: 40,
        search_start: 1916284264916,
        search_end: match max_range {
            Some(range) => 6553600000000.min(1916284264916 + range),
            _ => 1916285264916,
        },
    };
}

/// Request a field from the server. Supplies CLI options as query strings.
pub fn get_field(
    mode: &Mode,
    api_base: &str,
    username: &str,
    base: &Option<u32>,
    max_range: &Option<u128>,
    field: &Option<u128>,
) -> FieldClaim {
    let mut query_url = api_base.to_owned();
    query_url += &match mode {
        Mode::Detailed => "/claim/detailed",
        Mode::Niceonly => "/claim/niceonly",
    };
    query_url += &("?username=".to_owned() + &username.to_string());
    if let Some(base_val) = base {
        query_url += &("&base=".to_owned() + &base_val.to_string());
    }
    if let Some(max_range_val) = max_range {
        query_url += &("&max_range=".to_owned() + &max_range_val.to_string());
    }
    if let Some(field_id_val) = field {
        query_url += &("&field=".to_owned() + &field_id_val.to_string());
    }
    query_url += &("&max_base=".to_owned() + &MAX_SUPPORTED_BASE.to_string());

    let response = reqwest::blocking::get(&query_url).unwrap_or_else(|e| panic!("Error: {}", e));
    match response.json::<FieldClaim>() {
        Ok(claim_data) => claim_data,
        Err(e) => panic!("Error: {}", e),
    }
}

/// Submit field results to the server. Panic if there is an error.
pub fn submit_field(mode: &Mode, api_base: &str, submit_data: FieldSubmit) {
    let url = match mode {
        Mode::Detailed => format!("{}/submit/detailed", api_base),
        Mode::Niceonly => format!("{}/submit/niceonly", api_base),
    };

    let response = reqwest::blocking::Client::new()
        .post(&url)
        .json(&submit_data)
        .send();
    match response {
        Ok(response) => {
            if response.status().is_success() {
                return; // 👍
            }
            match response.text() {
                Ok(msg) => panic!("Server returned an error: {}", msg),
                Err(_) => panic!("Server returned an error."),
            }
        }
        Err(e) => panic!("Network error: {}", e),
    }
}

/// Get the count of unique digits in a number's sqube when represented in a specific base.
pub fn get_num_uniques(num: Natural, base: u32) -> u32 {
    // create a boolean array that represents all possible digits
    let mut digits_indicator: Vec<bool> = vec![false; base as usize];

    // square the number, convert to base and save the digits
    let squared = (&num).pow(2);
    for digit in squared.to_digits_asc(&base) {
        digits_indicator[digit as usize] = true;
    }

    // cube, convert to base and save the digits
    let cubed = squared * num;
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

/// Quickly determine if a number is 100% nice.
/// Assumes we have already done residue class filtering.
pub fn get_is_nice(num: &Natural, base: &Natural) -> bool {
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

/// Get a list of residue filters for a base.
/// For more information: https://beautifulthorns.wixsite.com/home/post/progress-update-on-the-search-for-nice-numbers
pub fn get_residue_filter(base: u32) -> Vec<u32> {
    let target_residue = base * (base - 1) / 2 % (base - 1);
    (0..(base - 1))
        .filter(|num| (num.pow(2) + num.pow(3)) % (base - 1) == target_residue)
        .collect()
}

/// Given a range, return a list of 100% nice numbers.
pub fn get_nice_list(n_start: u128, n_end: u128, base: u32) -> Vec<u128> {
    let base_natural = Natural::from(base);
    let residue_filter = get_residue_filter(base);
    (n_start..n_end)
        .filter(|num| residue_filter.contains(&((num % (base as u128 - 1)) as u32)))
        .filter(|num| get_is_nice(&Natural::from(*num), &base_natural))
        .collect()
}

/// Given a range, return two maps:
/// - A map of near misses and the number of unique digits in the sqube of each.
/// - A map of integers [1,base] and the count of numbers with that many unique digits.
pub fn process_range_detailed(
    n_start: u128,
    n_end: u128,
    base: u32,
) -> (HashMap<u128, u32>, HashMap<u32, u32>) {
    // near_misses_cutoff: minimum number of uniques required for the number to be recorded
    let near_misses_cutoff: u32 = (base as f32 * 0.9) as u32;

    // near_miss_list: list of numbers with niceness ratio (uniques/base) above the cutoff
    let mut near_miss_list: Vec<u128> = Vec::new();

    // qty_uniques: the quantity of numbers with each possible niceness
    let mut qty_uniques: Vec<u32> = vec![0; base as usize];

    // loop for all items in range (try to optimize anything in here)
    for num in n_start..n_end {
        // get the number of uniques in the sqube
        let num_uniques: u32 = get_num_uniques(Natural::from(num), base);

        // check if it's nice enough to record in near_miss_list
        if num_uniques > near_misses_cutoff {
            near_miss_list.push(num);
        }

        // update our quantity distribution in qty_uniques
        qty_uniques[num_uniques as usize - 1] += 1;
    }

    // build the initial values (api expects it)
    let mut unique_count_map: HashMap<u32, u32> = HashMap::new();
    for (num, count) in qty_uniques.iter().enumerate() {
        unique_count_map.insert(num as u32 + 1, *count);
    }

    // buid out the miss map
    let mut near_miss_map: HashMap<u128, u32> = HashMap::new();
    for nm in near_miss_list.iter() {
        near_miss_map.insert(*nm, get_num_uniques(Natural::from(*nm), base));
    }

    // return it as a tuple
    return (near_miss_map, unique_count_map);
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
        Mode::Detailed => {
            let (near_misses, unique_count) = process_range_detailed(
                claim_data.search_start,
                claim_data.search_end,
                claim_data.base,
            );
            FieldSubmit {
                id: claim_data.id,
                username: &username,
                client_version: &CLIENT_VERSION,
                unique_count: Some(unique_count),
                near_misses: Some(near_misses),
                nice_list: None,
            }
        }
        Mode::Niceonly => FieldSubmit {
            id: claim_data.id,
            username: &username,
            client_version: &CLIENT_VERSION,
            unique_count: None,
            near_misses: None,
            nice_list: Some(get_nice_list(
                claim_data.search_start,
                claim_data.search_end,
                claim_data.base,
            )),
        },
    };

    if !quiet {
        println!("{:?}", submit_data);
    }
    if benchmark || verbose {
        println!("Elapsed time: {:.3?}", before.elapsed());
        println!(
            "Hash rate:    {:.3e}",
            (claim_data.search_end - claim_data.search_start) / before.elapsed().as_secs() as u128
        );
    }
    if !benchmark {
        submit_field(&mode, &api_base, submit_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_get_is_nice() {
        assert_eq!(
            get_is_nice(&Natural::from(68 as u128), &Natural::from(10 as u32)),
            false
        );
        assert_eq!(
            get_is_nice(&Natural::from(69 as u128), &Natural::from(10 as u32)),
            true
        );
        assert_eq!(
            get_is_nice(&Natural::from(70 as u128), &Natural::from(10 as u32)),
            false
        );
        assert_eq!(
            get_is_nice(
                &Natural::from(173583337834150 as u128),
                &Natural::from(44 as u32)
            ),
            false
        );
    }

    #[test]
    fn test_process_range_detailed() {
        assert_eq!(
            process_range_detailed(47, 100, 10),
            (
                HashMap::from([(69, 10),]),
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
                HashMap::from([]),
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

    #[test]
    fn test_get_residue_filter() {
        assert_eq!(get_residue_filter(10), Vec::from([0, 3, 6, 8]));
        assert_eq!(get_residue_filter(11), Vec::<u32>::new());
        assert_eq!(get_residue_filter(12), Vec::from([0, 10]));
        assert_eq!(get_residue_filter(13), Vec::from([5, 9]));
        assert_eq!(get_residue_filter(14), Vec::from([0, 12]));
        assert_eq!(get_residue_filter(15), Vec::<u32>::new());
        assert_eq!(get_residue_filter(16), Vec::from([0, 5, 9, 14]));
    }

    #[test]
    fn test_get_nice_list() {
        assert_eq!(get_nice_list(47, 100, 10), Vec::from([69,]));
        assert_eq!(get_nice_list(144, 329, 12), Vec::<u128>::new());
    }
}
