//! A module with "nice" calculation utilities.

use crate::api_com::FieldClaim;

use super::*;

#[allow(unused_assignments)]
pub fn process_niceonly_natural(claim_data: &FieldClaim) -> FieldSubmit {
    let base = claim_data.base;
    let base_natural = Natural::from(base);

    let target_residue = base * (base - 1) / 2 % (base - 1);
    let residue_filter: Vec<u32> = (0..(base - 1))
        .filter(|num| (num.pow(2) + num.pow(3)) % (base - 1) == target_residue)
        .collect();

    let mut nice_list = Vec::new();
    let mut num = &claim_data.search_start - Natural::ONE;
    let mut digits_indicator = [false; MAX_SUPPORTED_BASE as usize];

    'search_range: while num < claim_data.search_end {
        // increment num
        num += Natural::ONE;

        // check residue
        let (_quotient, remainder) = (&num).div_mod(&base_natural - Natural::ONE);
        let residue = u32::try_from(&remainder).unwrap();
        if !residue_filter.contains(&residue) {
            continue;
        }

        // zero out the indicator
        digits_indicator = [false; MAX_SUPPORTED_BASE as usize];

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
    fn test_process_niceonly_natural_b10() {
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
    fn test_process_niceonly_natural_b40() {
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
