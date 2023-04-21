//! A module with "nice" calculation utilities.

use crate::api_com::FieldClaim;

use super::*;

/// Process a field by aggregating statistics on the niceness of numbers in a range.
pub fn process_detailed_natural(claim_data: &FieldClaim) -> FieldSubmit {
    let base = claim_data.base;
    let base_natural = Natural::from(base);
    let near_misses_cutoff = (base as f32 * NEAR_MISS_CUTOFF_PERCENT) as u32;

    // output variables
    let mut unique_digits: u32;
    let mut near_misses: HashMap<Natural, u32> = HashMap::new();
    let mut unique_count_vec = vec![0; base as usize];

    // iterator variables
    let mut n: Natural;
    let mut num = claim_data.search_start.clone();
    let mut digits_indicator = [false; MAX_SUPPORTED_BASE as usize];

    while num < claim_data.search_end {
        // zero out the indicator
        digits_indicator.iter_mut().for_each(|x| *x = false);

        // square the number and save those digits
        let squared = (&num).pow(2);
        n = squared.clone();
        while n > 0 {
            let remainder = usize::try_from(&(n.div_assign_rem(&base_natural))).unwrap();
            digits_indicator[remainder] = true;
        }

        // cube the number and save those digits
        n = squared * &num;
        while n > 0 {
            let remainder = usize::try_from(&(n.div_assign_rem(&base_natural))).unwrap();
            digits_indicator[remainder] = true;
        }

        // count the digits, update the unique count
        unique_digits = digits_indicator.iter().filter(|&&x| x).count() as u32;
        unique_count_vec[unique_digits as usize] += 1;

        // save if the number is pretty nice
        if unique_digits > near_misses_cutoff {
            near_misses.insert(num.clone(), unique_digits);
        }

        // increment num
        num += Natural::ONE;
    }

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

/// Get a list of residue filters for a base.
/// For more information: https://beautifulthorns.wixsite.com/home/post/progress-update-on-the-search-for-nice-numbers
pub fn get_residue_filter(base: &u32) -> Vec<u32> {
    let target_residue = base * (base - 1) / 2 % (base - 1);
    (0..(base - 1))
        .filter(|num| (num.pow(2) + num.pow(3)) % (base - 1) == target_residue)
        .collect()
}

/// Process a field by looking for completely nice numbers.
/// Implements several optimizations over the detailed search.
pub fn process_niceonly_natural(claim_data: &FieldClaim) -> FieldSubmit {
    let base = claim_data.base;
    let base_natural = Natural::from(base);
    let base_natural_sub_one = Natural::from(base) - Natural::ONE;

    let residue_filter = get_residue_filter(&base);

    // output & iterator variables
    let mut nice_list = Vec::new();
    let mut num = &claim_data.search_start - Natural::ONE;
    let mut digits_indicator = [false; MAX_SUPPORTED_BASE as usize];

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
        nice_list.push(num.clone());
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

    #[test]
    fn process_detailed_natural_b10() {
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
            near_misses: Some(HashMap::from([(Natural::from(69 as u128), 10)])),
            nice_list: None,
        };
        assert_eq!(process_detailed_natural(&claim_data), submit_data);
    }

    #[test]
    fn process_detailed_natural_b40() {
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
        assert_eq!(process_detailed_natural(&claim_data), submit_data);
    }

    #[test]
    fn test_get_residue_filter() {
        assert_eq!(get_residue_filter(&10), Vec::from([0, 3, 6, 8]));
        assert_eq!(get_residue_filter(&11), Vec::<u32>::new());
        assert_eq!(get_residue_filter(&12), Vec::from([0, 10]));
        assert_eq!(get_residue_filter(&13), Vec::from([5, 9]));
        assert_eq!(get_residue_filter(&14), Vec::from([0, 12]));
        assert_eq!(get_residue_filter(&15), Vec::<u32>::new());
        assert_eq!(get_residue_filter(&16), Vec::from([0, 5, 9, 14]));
    }

    #[test]
    fn process_niceonly_natural_b10() {
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
            nice_list: Some(Vec::from([Natural::from(69 as u128)])),
        };
        assert_eq!(process_niceonly_natural(&claim_data), submit_data);
    }

    #[test]
    fn process_niceonly_natural_b40() {
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
        assert_eq!(process_niceonly_natural(&claim_data), submit_data);
    }
}
