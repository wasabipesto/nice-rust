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
            Vec::from([69,]),
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
            Vec::from([]),
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
fn test_process_range_niceonly() {
    assert_eq!(process_range_niceonly(47, 100, 10), Vec::from([69,]));
    assert_eq!(process_range_niceonly(144, 329, 12), Vec::<u128>::new());
}
