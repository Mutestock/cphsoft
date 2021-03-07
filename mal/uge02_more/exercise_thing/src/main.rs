trait RealSetTrait {
    fn contains(&self, real: f32) -> bool;
    fn union(&self, other: RealSet) -> RealSet;
    fn difference(&self, other: RealSet) -> RealSet;
    fn complement(&self) -> RealSet;
}

struct UnionRealSet {
    first: RealSet,
    second: RealSet
}

struct RealSet {
    min: f32,
    max: f32,
    count_max: bool,
    count_min: bool,
}

impl RealSetTrait for RealSet {
    fn contains(&self, real: f32) -> bool {
        if self.count_min && self.count_min && real > self.min && real < self.max {
            true
        } else if self.count_min && !(self.count_max) && real > self.min {
            true
        } else if !(self.count_min) && self.count_max && real < self.max {
            true
        } else if !(self.count_min) && !(self.count_max){
            true
        }
        else{
            false
        }
    }
    fn union(&self, other: Self) -> Self{
        unimplemented!()
    }

    fn difference(&self, other: Self) -> Self{
        unimplemented!()
    }

    fn complement(&self) -> Self{
        unimplemented!()
    }
}

impl RealSet {
    fn new(min: f32, max: f32, count_max: bool, count_min: bool) -> Self {
        if min < max {
            return Self {
                min: min,
                max: max,
                count_max: count_max,
                count_min: count_min,
            };
        }
        panic!("Min can't be larger than min");
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_max(){
        let RealSet = RealSet::new(6.2, 33.9, true, true);
        assert_eq!(RealSet.contains(12.2), true);
        assert_eq!(RealSet.contains(2.3), false);
    }
    #[test]
    fn max_not_min(){
        let RealSet = RealSet::new(6.2, 33.9, false, true);
        assert_eq!(RealSet.contains(69.2), true);
        assert_eq!(RealSet.contains(2.3), false);
    }
    #[test]
    fn not_max_min(){
        let RealSet = RealSet::new(6.2, 33.9, true, false);
        assert_eq!(RealSet.contains(2.1), true);
        assert_eq!(RealSet.contains(69.3), false);
    }
    #[test]
    fn not_max_not_min(){
        let RealSet = RealSet::new(6.2, 33.9, false, false);
        assert_eq!(RealSet.contains(-2.1), true);
    }
}
