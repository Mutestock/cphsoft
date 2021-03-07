pub fn _nand(a: bool, b: bool) -> bool {
    if !(a && b) {
        return true;
    }
    false
}

pub fn _xor(a: bool, b: bool) -> bool {
    if (a || b) && !(a && b) {
        return true;
    }
    false
}

pub fn _and(a: bool, b: bool) -> bool {
    if a && b {
        return true;
    }
    false
}

pub fn _or(a: bool, b: bool) -> bool {
    if a || b {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn nand01() {
        assert_eq!(_nand(true, false), true);
    }

    #[test]
    fn nand02() {
        assert_eq!(_nand(true, true), false);
    }

    #[test]
    fn nand03() {
        assert_eq!(_nand(false, false), true);
    }

    #[test]
    fn xor01() {
        assert_eq!(_xor(false, false), false);
    }

    #[test]
    fn xor02() {
        assert_eq!(_xor(false, true), true);
    }

    #[test]
    fn xor03() {
        assert_eq!(_xor(true, true), false);
    }

    #[test]
    fn and01() {
        assert_eq!(_and(true, true), true);
    }

    #[test]
    fn and02() {
        assert_eq!(_and(false, true), false);
    }
    #[test]
    fn and03() {
        assert_eq!(_and(false, false), false);
    }

    #[test]
    fn or01() {
        assert_eq!(_or(true, true), true);
    }

    #[test]
    fn or02() {
        assert_eq!(_or(true, false), true);
    }
    #[test]
    fn or03() {
        assert_eq!(_or(false, false), false);
    }
}
