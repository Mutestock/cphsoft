use crate::gates::{_and, _nand, _or, _xor};

// Where (sum, carry)
pub fn half_adder(a: bool, b: bool) -> (bool, bool) {
    if _and(a, b) {
        return (false, true);
    } else if _xor(a, b) {
        return (true, false);
    }
    (false, false)
}

// Where (sum, carry)
pub fn full_adder(a: bool, b: bool, c_in: bool) -> (bool, bool) {
    let res01 = half_adder(a, b);
    let res02 = half_adder(c_in, res01.0);
    (res02.0, _or(res02.1, res01.1))
}

// Where string is a hexideciamal result
pub fn four_bit_digit(a: bool, b: bool, c: bool, d: bool) -> String {
    let binary_string = boolean_sequence_to_bit_string(vec![a, b, c, d]);
    let num = isize::from_str_radix(&binary_string, 2).unwrap();
    format!("{:x}", num)
}

pub fn boolean_sequence_to_bit_string(vec: Vec<bool>) -> String {
    let mut res = String::new();
    for value in vec {
        if value {
            res.push('1')
        } else {
            res.push('0')
        }
    }
    res.chars().rev().collect::<String>()
}

pub fn two_times_four_bit_addition(
    a: (bool, bool, bool, bool),
    b: (bool, bool, bool, bool),
) -> String {
    let res01 = half_adder(a.0, b.0);
    let res02 = full_adder(a.1, b.1, res01.1);
    let res03 = full_adder(a.2, b.2, res02.1);
    let res04 = full_adder(a.3, b.3, res03.1);

    four_bit_digit(res01.0, res02.0, res03.0, res04.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn half_adder01() {
        assert_eq!(half_adder(true, true), (false, true))
    }

    #[test]
    fn half_adder02() {
        assert_eq!(half_adder(false, true), (true, false))
    }

    #[test]
    fn half_adder03() {
        assert_eq!(half_adder(false, false), (false, false))
    }

    #[test]
    fn full_adder01() {
        assert_eq!(full_adder(false, false, false), (false, false))
    }

    #[test]
    fn full_adder02() {
        assert_eq!(full_adder(false, false, true), (true, false))
    }

    #[test]
    fn full_adder03() {
        assert_eq!(full_adder(false, true, false), (true, false))
    }

    #[test]
    fn full_adder04() {
        assert_eq!(full_adder(false, true, true), (false, true))
    }

    #[test]
    fn full_adder05() {
        assert_eq!(full_adder(true, true, true), (true, true))
    }

    #[test]
    fn full_adder06() {
        assert_eq!(full_adder(true, true, false), (false, true))
    }

    #[test]
    fn four_bit_digit01() {
        assert_eq!(four_bit_digit(true, false, false, false), "1")
    }
    #[test]
    fn four_bit_digit02() {
        assert_eq!(four_bit_digit(true, true, true, true), "f")
    }

    #[test]
    fn four_bit_digit03() {
        assert_eq!(four_bit_digit(false, true, false, true), "a")
    }

    #[test]
    fn two_times_four_bit_addition01() {
        assert_eq!(
            two_times_four_bit_addition((false, false, false, false), (false, false, false, false)),
            "0"
        )
    }

    #[test]
    fn two_times_four_bit_addition02() {
        assert_eq!(
            two_times_four_bit_addition((true, false, false, false), (false, false, false, false)),
            "1"
        )
    }

    #[test]
    fn two_times_four_bit_addition03() {
        assert_eq!(
            two_times_four_bit_addition((true, false, false, false), (true, false, false, false)),
            "2"
        )
    }

    #[test]
    fn two_times_four_bit_addition04() {
        assert_eq!(
            two_times_four_bit_addition((false, true, false, false), (false, true, false, false)),
            "4"
        )
    }

    #[test]
    fn two_times_four_bit_addition05() {
        assert_eq!(
            two_times_four_bit_addition((true, false, false, false), (false, true, true, true)),
            "f"
        )
    }

    #[test]
    fn two_times_four_bit_addition06() {
        assert_eq!(
            two_times_four_bit_addition((false, false, true, false), (false, false, false, true)),
            "c"
        )
    }

    #[test]
    fn two_times_four_bit_addition07() {
        assert_eq!(
            two_times_four_bit_addition((false, false, true, true), (false, false, true, true)),
            "8"
        )
    }
}
