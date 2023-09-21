//! A module with "nice" calculation utilities.
//! We will iterate over n as a Natural directly so we can process arbitrarily high ranges.

use super::*;

/// Process a field by aggregating statistics on the niceness of numbers in a range.
pub fn process_detailed(claim_data: &FieldClaim) -> FieldSubmit {
    let base = claim_data.base;
    let base_natural = Natural::from(base);
    let near_misses_cutoff = (base as f32 * NEAR_MISS_CUTOFF_PERCENT) as u32;

    // output variables
    let mut unique_digits: u32;
    let mut near_misses: HashMap<String, u32> = HashMap::new();
    let mut unique_count_vec = vec![0; base as usize];

    // iterator variables
    let mut n: Natural;
    let mut num = claim_data.search_start.clone();
    let mut digits_indicator = [false; MAX_SUPPORTED_BASE_HIGH as usize];

    while num < claim_data.search_end {
        // zero out the indicator
        digits_indicator.iter_mut().for_each(|x| *x = false);

        // square the number and save those digits
        n = (&num).pow(2);
        while n > 0 {
            let remainder = usize::try_from(&(n.div_assign_rem(&base_natural))).unwrap();
            digits_indicator[remainder] = true;
        }

        // cube the number and save those digits
        n = (&num).pow(3);
        while n > 0 {
            let remainder = usize::try_from(&(n.div_assign_rem(&base_natural))).unwrap();
            digits_indicator[remainder] = true;
        }

        // count the digits, update the unique count
        unique_digits = digits_indicator.iter().filter(|&&x| x).count() as u32;
        unique_count_vec[unique_digits as usize - 1] += 1;

        // save if the number is pretty nice
        if unique_digits > near_misses_cutoff {
            near_misses.insert(num.to_string(), unique_digits);
        }

        // increment num
        num += Natural::ONE;
    }

    // sum unique counts from vec
    let unique_count = unique_count_vec
        .iter()
        .enumerate()
        .map(|(i, &x)| (i as u32 + 1, x))
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
pub fn process_niceonly(claim_data: &FieldClaim) -> FieldSubmit {
    let base = claim_data.base;
    let base_natural = Natural::from(base);
    let base_natural_sub_one = Natural::from(base) - Natural::ONE;

    let residue_filter = get_residue_filter(&base);

    // output & iterator variables
    let mut nice_list = Vec::new();
    let mut num = &claim_data.search_start - Natural::ONE;
    let mut digits_indicator = [false; MAX_SUPPORTED_BASE_HIGH as usize];

    'search_range: while num < claim_data.search_end {
        // increment num
        num += Natural::ONE;

        // check residue
        let remainder = (&num).mod_op(&base_natural_sub_one);
        let residue = u32::try_from(&remainder).unwrap();
        if !residue_filter.contains(&residue) {
            continue;
        }

        // zero out the indicator
        digits_indicator.iter_mut().for_each(|x| *x = false);

        // square the number and check those digits
        let squared = (&num).pow(2);
        let mut n = squared.clone();
        while n > 0 {
            let remainder = usize::try_from(&(n.div_assign_rem(&base_natural))).unwrap();
            if digits_indicator[remainder] {
                continue 'search_range;
            }
            digits_indicator[remainder] = true;
        }

        // cube the number and check those digits
        let mut n = squared * &num;
        while n > 0 {
            let remainder = usize::try_from(&(n.div_assign_rem(&base_natural))).unwrap();
            if digits_indicator[remainder] {
                continue 'search_range;
            }
            digits_indicator[remainder] = true;
        }

        // save the number!
        nice_list.push(num.to_string());
    }

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
        assert_eq!(process_niceonly(&claim_data), submit_data);
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
        assert_eq!(process_niceonly(&claim_data), submit_data);
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
        assert_eq!(process_niceonly(&claim_data), submit_data);
    }

    #[test]
    fn process_niceonly_b120() {
        let claim_data = FieldClaim {
            id: 0,
            username: "benchmark".to_owned(),
            base: 120,
            search_start: Natural::from_str("16117196090075248994613996554363597629408239219454")
                .unwrap(),
            search_end: Natural::from_str("16117196090075248994613996554363597629408239319454")
                .unwrap(),
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
        assert_eq!(process_niceonly(&claim_data), submit_data);
    }
}
