//! A module for deaing with residue filters
//! For more information: https://beautifulthorns.wixsite.com/home/post/progress-update-on-the-search-for-nice-numbers

/// Get a list of residue filters for a base.
pub fn get_residue_filter(base: &u32) -> Vec<u32> {
    let target_residue = base * (base - 1) / 2 % (base - 1);
    (0..(base - 1))
        .filter(|num| (num.pow(2) + num.pow(3)) % (base - 1) == target_residue)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_residue_filter() {
        assert_eq!(get_residue_filter(&10), Vec::from([0, 3, 6, 8]));
        assert_eq!(get_residue_filter(&11), Vec::<u32>::new());
        assert_eq!(get_residue_filter(&12), Vec::from([0, 10]));
        assert_eq!(get_residue_filter(&13), Vec::from([5, 9]));
        assert_eq!(get_residue_filter(&14), Vec::from([0, 12]));
        assert_eq!(get_residue_filter(&15), Vec::<u32>::new());
        assert_eq!(get_residue_filter(&16), Vec::from([0, 5, 9, 14]));
        assert_eq!(get_residue_filter(&17), Vec::from([7]));
        assert_eq!(get_residue_filter(&18), Vec::from([0, 16]));
        assert_eq!(get_residue_filter(&19), Vec::<u32>::new());
        assert_eq!(get_residue_filter(&20), Vec::from([0, 18]));
        assert_eq!(get_residue_filter(&21), Vec::from([5, 9]));
        assert_eq!(get_residue_filter(&22), Vec::from([0, 6, 14, 20]));
        assert_eq!(get_residue_filter(&23), Vec::<u32>::new());
        assert_eq!(get_residue_filter(&24), Vec::from([0, 22]));
        assert_eq!(get_residue_filter(&25), Vec::from([2, 3, 6, 11, 14, 18]));
        assert_eq!(get_residue_filter(&26), Vec::from([0, 5, 10, 15, 20, 24]));
        assert_eq!(get_residue_filter(&27), Vec::<u32>::new());
        assert_eq!(get_residue_filter(&28), Vec::from([0, 9, 18, 26]));
        assert_eq!(get_residue_filter(&29), Vec::from([13, 21]));
        assert_eq!(get_residue_filter(&30), Vec::from([0, 28]));
        assert_eq!(get_residue_filter(&40), Vec::from([0, 12, 26, 38]));
        assert_eq!(
            get_residue_filter(&50),
            Vec::from([0, 7, 14, 21, 28, 35, 42, 48])
        );
        assert_eq!(get_residue_filter(&60), Vec::from([0, 58]));
        assert_eq!(get_residue_filter(&70), Vec::from([0, 23, 45, 68]));
        assert_eq!(get_residue_filter(&80), Vec::from([0, 78]));
        assert_eq!(get_residue_filter(&90), Vec::from([0, 88]));
        assert_eq!(
            get_residue_filter(&100),
            Vec::from([0, 21, 33, 44, 54, 66, 87, 98])
        );
        assert_eq!(get_residue_filter(&110), Vec::from([0, 108]));
        assert_eq!(get_residue_filter(&111), Vec::<u32>::new());
        assert_eq!(get_residue_filter(&112), Vec::from([0, 36, 74, 110]));
        assert_eq!(get_residue_filter(&113), Vec::from([7, 55]));
        assert_eq!(get_residue_filter(&114), Vec::from([0, 112]));
        assert_eq!(get_residue_filter(&115), Vec::<u32>::new());
        assert_eq!(get_residue_filter(&116), Vec::from([0, 45, 69, 114]));
        assert_eq!(get_residue_filter(&117), Vec::from([29, 57]));
        assert_eq!(
            get_residue_filter(&118),
            Vec::from([0, 12, 26, 39, 51, 78, 90, 116])
        );
        assert_eq!(get_residue_filter(&119), Vec::<u32>::new());
        assert_eq!(get_residue_filter(&120), Vec::from([0, 34, 84, 118]));
    }
}
