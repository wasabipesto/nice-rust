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
pub fn process_detailed(claim_data: &FieldClaim) -> FieldSubmit {
    let base = claim_data.base;
    let search_start = u128::try_from(&claim_data.search_start).unwrap();
    let search_end = u128::try_from(&claim_data.search_end).unwrap();

    // process the range and collect num_uniques for each item in the range
    let result_map: HashMap<u128, u32> = (search_start..search_end)
        .into_iter()
        .map(|num| (num, get_num_uniques(num, base)))
        .collect();

    // collect the near misses from the result map
    let near_misses_cutoff = (base as f32 * NEAR_MISS_CUTOFF_PERCENT) as u32;
    let near_misses: HashMap<String, u32> = result_map
        .into_iter()
        .filter(|&(_, value)| value > near_misses_cutoff)
        .map(|(num, value)| (num.to_string(), value))
        .collect();

    // collect the distribution of uniqueness across the result map
    let unique_count: HashMap<u32, u32> = (1..base)
        .into_iter()
        .map(|i| {
            let count = result_map.values().filter(|&&v| v == i).count() as u32;
            (i, count)
        })
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

/// Process a field by looking for completely nice numbers.
/// Implements several optimizations over the detailed search.
pub fn process_niceonly(claim_data: &FieldClaim) -> FieldSubmit {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

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
        assert_eq!(process_detailed(&claim_data), submit_data);
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
        assert_eq!(process_detailed(&claim_data), submit_data);
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
        assert_eq!(process_detailed(&claim_data), submit_data);
    }

    #[test]
    fn process_detailed_b120() {
        let claim_data = FieldClaim {
            id: 0,
            username: "benchmark".to_owned(),
            base: 120,
            search_start: Natural::from_str("16117196090075248994613996554363597629408239219454")
                .unwrap(),
            search_end: Natural::from_str("16117196090075248994613996554363597629408239229454")
                .unwrap(),
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
                (36, 0),
                (37, 0),
                (38, 0),
                (39, 0),
                (40, 0),
                (41, 0),
                (42, 0),
                (43, 0),
                (44, 0),
                (45, 0),
                (46, 0),
                (47, 0),
                (48, 0),
                (49, 0),
                (50, 0),
                (51, 0),
                (52, 0),
                (53, 0),
                (54, 0),
                (55, 3),
                (56, 4),
                (57, 12),
                (58, 25),
                (59, 52),
                (60, 96),
                (61, 198),
                (62, 325),
                (63, 525),
                (64, 708),
                (65, 972),
                (66, 1206),
                (67, 1256),
                (68, 1189),
                (69, 1045),
                (70, 884),
                (71, 619),
                (72, 399),
                (73, 246),
                (74, 136),
                (75, 51),
                (76, 29),
                (77, 16),
                (78, 2),
                (79, 0),
                (80, 2),
                (81, 0),
                (82, 0),
                (83, 0),
                (84, 0),
                (85, 0),
                (86, 0),
                (87, 0),
                (88, 0),
                (89, 0),
                (90, 0),
                (91, 0),
                (92, 0),
                (93, 0),
                (94, 0),
                (95, 0),
                (96, 0),
                (97, 0),
                (98, 0),
                (99, 0),
                (100, 0),
                (101, 0),
                (102, 0),
                (103, 0),
                (104, 0),
                (105, 0),
                (106, 0),
                (107, 0),
                (108, 0),
                (109, 0),
                (110, 0),
                (111, 0),
                (112, 0),
                (113, 0),
                (114, 0),
                (115, 0),
                (116, 0),
                (117, 0),
                (118, 0),
                (119, 0),
                (120, 0),
            ])),
            near_misses: Some(HashMap::new()),
            nice_list: None,
        };
        assert_eq!(process_detailed(&claim_data), submit_data);
    }
}
