//! A module for calculating the apprpriate range for each base.

use super::*;

/// Get the range of possible values for a base.
pub fn get_base_range(base: u32) -> (Natural, Natural) {
    let b = Natural::from(base);
    let k = (base / 5) as u64;

    match base % 5 {
        0 => (b.clone().pow(3 * k - 1).ceiling_root(3), b.pow(k)),
        1 => (Natural::ZERO, Natural::ZERO),
        2 => (b.clone().pow(k), b.pow(3 * k + 1).floor_root(3)),
        3 => (
            b.clone().pow(3 * k + 1).ceiling_root(3),
            b.pow(2 * k + 1).floor_root(2),
        ),
        4 => (
            b.clone().pow(2 * k + 1).ceiling_root(2),
            b.pow(3 * k + 2).floor_root(3),
        ),
        _ => (Natural::ZERO, Natural::ZERO),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_get_base_range() {
        assert_eq!(
            get_base_range(4),
            (Natural::from(2 as u32), Natural::from(2 as u32))
        );
        assert_eq!(
            get_base_range(5),
            (Natural::from(3 as u32), Natural::from(5 as u32))
        );
        assert_eq!(get_base_range(6), (Natural::ZERO, Natural::ZERO));
        assert_eq!(
            get_base_range(7),
            (Natural::from(7 as u32), Natural::from(13 as u32))
        );
        assert_eq!(
            get_base_range(8),
            (Natural::from(16 as u32), Natural::from(22 as u32))
        );
        assert_eq!(
            get_base_range(9),
            (Natural::from(27 as u32), Natural::from(38 as u32))
        );
        assert_eq!(
            get_base_range(10),
            (Natural::from(47 as u32), Natural::from(100 as u32))
        );
        assert_eq!(
            get_base_range(20),
            (Natural::from(58945 as u32), Natural::from(160000 as u32))
        );
        assert_eq!(
            get_base_range(30),
            (
                Natural::from(234613921 as u32),
                Natural::from(729000000 as u32)
            )
        );
        assert_eq!(
            get_base_range(40),
            (
                Natural::from(1916284264916 as u64),
                Natural::from(6553600000000 as u64)
            )
        );
        assert_eq!(
            get_base_range(50),
            (
                Natural::from(26507984537059635 as u64),
                Natural::from(97656250000000000 as u64)
            )
        );
        // start getting rounding errors here
        assert_eq!(
            get_base_range(60),
            (
                Natural::from(556029612114824200908 as u128),
                Natural::from(2176782336000000000000 as u128)
            )
        );
        assert_eq!(
            get_base_range(70),
            (
                Natural::from(16456591172673850596148008 as u128),
                Natural::from(67822307284900000000000000 as u128)
            )
        );
        assert_eq!(
            get_base_range(80),
            (
                Natural::from(653245554420798943087177909799 as u128),
                Natural::from(2814749767106560000000000000000 as u128)
            )
        );
        assert_eq!(
            get_base_range(90),
            (
                Natural::from(33492764832792484045981163311105668 as u128),
                Natural::from(150094635296999121000000000000000000 as u128)
            )
        );
        // around here we run into the limits of u128
        assert_eq!(
            get_base_range(100),
            (
                Natural::from_str("2154434690031883721759293566519350495260").unwrap(),
                Natural::from_str("10000000000000000000000000000000000000000").unwrap()
            )
        );
        assert_eq!(
            get_base_range(110),
            (
                Natural::from_str("169892749571608053239273597713205371466519752").unwrap(),
                Natural::from_str("814027493868397611133210000000000000000000000").unwrap()
            )
        );
        assert_eq!(
            get_base_range(120),
            (
                Natural::from_str("16117196090075248994613996554363597629408239219454").unwrap(),
                Natural::from_str("79496847203390844133441536000000000000000000000000").unwrap()
            )
        );
        // run through the mod5 series at a the high end to check everything is still good
        assert_eq!(get_base_range(121), (Natural::ZERO, Natural::ZERO));
        assert_eq!(
            get_base_range(122),
            (
                Natural::from_str("118205024187370033135932935819405317049548439289856").unwrap(),
                Natural::from_str("586258581805989694050980431834549184603056531020210").unwrap()
            )
        );
        assert_eq!(
            get_base_range(123),
            (
                Natural::from_str("715085071699820536699499456671007010425915160419662").unwrap(),
                Natural::from_str("1594686179043939546502781159240976178904795301633107").unwrap()
            )
        );
        assert_eq!(
            get_base_range(124),
            (
                Natural::from_str("1944604500263970232242123784503740458789493393829926").unwrap(),
                Natural::from_str("4342450740818512904293955173690913927483946149220888").unwrap()
            )
        );
        assert_eq!(
            get_base_range(125),
            (
                Natural::from_str("5293955920339377119177015629247762262821197509765625").unwrap(),
                Natural::from_str("26469779601696885595885078146238811314105987548828125").unwrap()
            )
        );
    }
}
