//! A module with "nice" calculation utilities.
//! We will iterate over n as u128 (max 3.4e38), but expand it into Natural for n^2 and n^3.
//! That means we can go up through base 97 (5.6e37 to 2.6e38) but not base 98 (3.1e38 to 6.7e38).

use super::*;

/// Get the count of unique digits in a number's sqube when represented in a specific base.
pub fn get_num_uniques(num: u128, base: u32) -> u32 {
    let num = Natural::from(num);

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

/// Process a field by aggregating statistics on the niceness of numbers in a range.
pub fn process_detailed(claim_data: &FieldClaim, parallel: bool) -> FieldSubmit {
    let base = claim_data.base;
    let search_start = u128::try_from(&claim_data.search_start).unwrap();
    let search_end = u128::try_from(&claim_data.search_end).unwrap();

    // process the range and collect num_uniques for each item in the range
    let result_map: HashMap<u128, u32> = match parallel {
        false => (search_start..search_end)
            .into_iter()
            .map(|num| (num, get_num_uniques(num, base)))
            .collect(),
        true => (search_start..search_end)
            .into_par_iter()
            .map(|num| (num, get_num_uniques(num, base)))
            .collect(),
    };

    // collect the near misses from the result map
    let near_misses_cutoff = (base as f32 * NEAR_MISS_CUTOFF_PERCENT) as u32;
    let near_misses: HashMap<String, u32> = result_map
        .clone()
        .into_iter()
        .filter(|&(_, value)| value > near_misses_cutoff)
        .map(|(num, value)| (num.to_string(), value))
        .collect();

    // collect the distribution of uniqueness across the result map
    let unique_count: HashMap<u32, u32> = (1..=base)
        .into_iter()
        .map(|i| (i, result_map.values().filter(|&&v| v == i).count() as u32))
        .collect();

    return FieldSubmit {
        id: claim_data.id,
        username: claim_data.username.clone(),
        client_version: CLIENT_VERSION.to_string(),
        unique_count: Some(unique_count),
        near_misses: Some(near_misses),
        nice_list: None,
    };
}

/// Quickly determine if a number is 100% nice.
/// Assumes we have already done residue class filtering.
pub fn get_is_nice(num: u128, base: u32) -> bool {
    let num = Natural::from(num);
    let base_natural = Natural::from(base);

    // create a boolean array that represents all possible digits
    let mut digits_indicator = [false; MAX_SUPPORTED_BASE_NORMAL as usize];

    // square the number and check those digits
    let squared = (&num).pow(2);
    let mut n = squared.clone();
    while n > 0 {
        let remainder = usize::try_from(&(n.div_assign_rem(&base_natural))).unwrap();
        if digits_indicator[remainder] {
            return false;
        }
        digits_indicator[remainder] = true;
    }

    // cube the number and check those digit
    let mut n = squared * num;
    while n > 0 {
        let remainder = usize::try_from(&(n.div_assign_rem(&base_natural))).unwrap();
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

/// Process a field by looking for completely nice numbers.
/// Implements several optimizations over the detailed search.
pub fn process_niceonly(claim_data: &FieldClaim, parallel: bool) -> FieldSubmit {
    let base = claim_data.base;
    let search_start = u128::try_from(&claim_data.search_start).unwrap();
    let search_end = u128::try_from(&claim_data.search_end).unwrap();
    let residue_filter = get_residue_filter(base);

    let nice_list = match parallel {
        false => (search_start..search_end)
            .into_iter()
            .filter(|num| residue_filter.contains(&((num % (base as u128 - 1)) as u32)))
            .filter(|num| get_is_nice(*num, base))
            .map(|num| num.to_string())
            .collect(),
        true => (search_start..search_end)
            .into_par_iter()
            .filter(|num| residue_filter.contains(&((num % (base as u128 - 1)) as u32)))
            .filter(|num| get_is_nice(*num, base))
            .map(|num| num.to_string())
            .collect(),
    };

    return FieldSubmit {
        id: claim_data.id,
        username: claim_data.username.clone(),
        client_version: CLIENT_VERSION.to_string(),
        unique_count: None,
        near_misses: None,
        nice_list: Some(nice_list),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_detailed_b10() {
        let claim_data = FieldClaim {
            id: 0,
            username: "benchmark".to_owned(),
            base: 10,
            search_start: Natural::from(47 as u128),
            search_end: Natural::from(100 as u128),
            search_range: Natural::from(53 as u128),
        };
        let submit_data = FieldSubmit {
            id: claim_data.id.clone(),
            username: claim_data.username.clone(),
            client_version: CLIENT_VERSION.to_string(),
            unique_count: Some(HashMap::from([
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
            ])),
            near_misses: Some(HashMap::from([("69".to_string(), 10)])),
            nice_list: None,
        };
        assert_eq!(process_detailed(&claim_data, true), submit_data);
    }

    #[test]
    fn process_detailed_b40() {
        let claim_data = FieldClaim {
            id: 0,
            username: "benchmark".to_owned(),
            base: 40,
            search_start: Natural::from(916284264916 as u128),
            search_end: Natural::from(916284264916 + 10000 as u128),
            search_range: Natural::from(10000 as u128),
        };
        let submit_data = FieldSubmit {
            id: claim_data.id.clone(),
            username: claim_data.username.clone(),
            client_version: CLIENT_VERSION.to_string(),
            unique_count: Some(HashMap::from([
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (6, 0),
                (7, 0),
                (8, 0),
                (9, 0),
                (10, 0),
                (11, 0),
                (12, 0),
                (13, 0),
                (14, 0),
                (15, 0),
                (16, 0),
                (17, 0),
                (18, 1),
                (19, 13),
                (20, 40),
                (21, 176),
                (22, 520),
                (23, 1046),
                (24, 1710),
                (25, 2115),
                (26, 1947),
                (27, 1322),
                (28, 728),
                (29, 283),
                (30, 83),
                (31, 13),
                (32, 3),
                (33, 0),
                (34, 0),
                (35, 0),
                (36, 0),
                (37, 0),
                (38, 0),
                (39, 0),
                (40, 0),
            ])),
            near_misses: Some(HashMap::new()),
            nice_list: None,
        };
        assert_eq!(process_detailed(&claim_data, true), submit_data);
    }

    #[test]
    fn process_detailed_b80() {
        let claim_data = FieldClaim {
            id: 0,
            username: "benchmark".to_owned(),
            base: 80,
            search_start: Natural::from(653245554420798943087177909799 as u128),
            search_end: Natural::from(653245554420798943087177909799 + 10000 as u128),
            search_range: Natural::from(10000 as u128),
        };
        let submit_data = FieldSubmit {
            id: claim_data.id.clone(),
            username: claim_data.username.clone(),
            client_version: CLIENT_VERSION.to_string(),
            unique_count: Some(HashMap::from([
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (6, 0),
                (7, 0),
                (8, 0),
                (9, 0),
                (10, 0),
                (11, 0),
                (12, 0),
                (13, 0),
                (14, 0),
                (15, 0),
                (16, 0),
                (17, 0),
                (18, 0),
                (19, 0),
                (20, 0),
                (21, 0),
                (22, 0),
                (23, 0),
                (24, 0),
                (25, 0),
                (26, 0),
                (27, 0),
                (28, 0),
                (29, 0),
                (30, 0),
                (31, 0),
                (32, 0),
                (33, 0),
                (34, 0),
                (35, 0),
                (36, 1),
                (37, 6),
                (38, 14),
                (39, 62),
                (40, 122),
                (41, 263),
                (42, 492),
                (43, 830),
                (44, 1170),
                (45, 1392),
                (46, 1477),
                (47, 1427),
                (48, 1145),
                (49, 745),
                (50, 462),
                (51, 242),
                (52, 88),
                (53, 35),
                (54, 19),
                (55, 7),
                (56, 1),
                (57, 0),
                (58, 0),
                (59, 0),
                (60, 0),
                (61, 0),
                (62, 0),
                (63, 0),
                (64, 0),
                (65, 0),
                (66, 0),
                (67, 0),
                (68, 0),
                (69, 0),
                (70, 0),
                (71, 0),
                (72, 0),
                (73, 0),
                (74, 0),
                (75, 0),
                (76, 0),
                (77, 0),
                (78, 0),
                (79, 0),
                (80, 0),
            ])),
            near_misses: Some(HashMap::new()),
            nice_list: None,
        };
        assert_eq!(process_detailed(&claim_data, true), submit_data);
    }

    #[test]
    fn process_niceonly_b10() {
        let claim_data = FieldClaim {
            id: 0,
            username: "benchmark".to_owned(),
            base: 10,
            search_start: Natural::from(47 as u128),
            search_end: Natural::from(100 as u128),
            search_range: Natural::from(53 as u128),
        };
        let submit_data = FieldSubmit {
            id: claim_data.id.clone(),
            username: claim_data.username.clone(),
            client_version: CLIENT_VERSION.to_string(),
            unique_count: None,
            near_misses: None,
            nice_list: Some(Vec::from(["69".to_string()])),
        };
        assert_eq!(process_niceonly(&claim_data, true), submit_data);
    }

    #[test]
    fn process_niceonly_b40() {
        let claim_data = FieldClaim {
            id: 0,
            username: "benchmark".to_owned(),
            base: 40,
            search_start: Natural::from(916284264916 as u128),
            search_end: Natural::from(916284264916 + 10000 as u128),
            search_range: Natural::from(10000 as u128),
        };
        let submit_data = FieldSubmit {
            id: claim_data.id.clone(),
            username: claim_data.username.clone(),
            client_version: CLIENT_VERSION.to_string(),
            unique_count: None,
            near_misses: None,
            nice_list: Some(Vec::new()),
        };
        assert_eq!(process_niceonly(&claim_data, true), submit_data);
    }

    #[test]
    fn process_niceonly_b80() {
        let claim_data = FieldClaim {
            id: 0,
            username: "benchmark".to_owned(),
            base: 80,
            search_start: Natural::from(653245554420798943087177909799 as u128),
            search_end: Natural::from(653245554420798943087177909799 + 10000 as u128),
            search_range: Natural::from(10000 as u128),
        };
        let submit_data = FieldSubmit {
            id: claim_data.id.clone(),
            username: claim_data.username.clone(),
            client_version: CLIENT_VERSION.to_string(),
            unique_count: None,
            near_misses: None,
            nice_list: Some(Vec::new()),
        };
        assert_eq!(process_niceonly(&claim_data, true), submit_data);
    }
}
