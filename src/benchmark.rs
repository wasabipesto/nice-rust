use std::collections::HashMap;

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

// get the number of unique digits in the concatenated sqube of a specified number
fn get_num_uniques(num: u128, base: u32) -> u32 {
    let b = base as u128;
    let mut sqube = number_to_base(num.pow(2), b);
    sqube.append(&mut number_to_base(num.pow(3), b));
    sqube.sort();
    sqube.dedup();
    return sqube.len() as u32;
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

fn main() {
    // search for near_misses and qty_uniques
    let (
        _near_misses, 
        _qty_uniques
    ) = search_range(
        1625364,
        2760487,
        24,
    );
}