pub trait Specification<T> {
    fn is_satisfied_by(&self, arg: &T) -> bool;
}

impl<T> std::fmt::Debug for dyn Specification<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

pub struct OrSpecification<T> {
    spec1: Box<dyn Specification<T>>,
    spec2: Box<dyn Specification<T>>,
}

impl<T> Specification<T> for OrSpecification<T> {
    fn is_satisfied_by(&self, arg: &T) -> bool {
        self.spec1.is_satisfied_by(arg) || self.spec2.is_satisfied_by(arg)
    }
}

pub struct AndSpecification<T> {
    pub spec1: Box<dyn Specification<T>>,
    pub spec2: Box<dyn Specification<T>>,
}

impl<T> Specification<T> for AndSpecification<T> {
    fn is_satisfied_by(&self, arg: &T) -> bool {
        self.spec1.is_satisfied_by(arg) && self.spec2.is_satisfied_by(arg)
    }
}

pub struct NotSpecification<T> {
    pub spec: Box<dyn Specification<T>>,
}

impl<T> Specification<T> for NotSpecification<T> {
    fn is_satisfied_by(&self, arg: &T) -> bool {
        !self.spec.is_satisfied_by(arg)
    }
}

#[test]
fn test_specification() {
    macro_rules! specification {
        ($t:ident) => {
            impl Specification<$t> for $t {
                fn is_satisfied_by(&self, arg: &$t) -> bool {
                    *self == *arg
                }
            }
        };
    }
    specification!(i32);

    let one_specification = 1;
    assert!(one_specification.is_satisfied_by(&1));
    assert!(!one_specification.is_satisfied_by(&2));

    let two_specification = 2;
    let or_specification = OrSpecification {
        spec1: Box::new(one_specification),
        spec2: Box::new(two_specification),
    };
    assert!(or_specification.is_satisfied_by(&1));
    assert!(!or_specification.is_satisfied_by(&3));

    let and_specification1 = AndSpecification {
        spec1: Box::new(one_specification),
        spec2: Box::new(two_specification),
    };
    assert!(!and_specification1.is_satisfied_by(&1));

    let and_specification2 = AndSpecification {
        spec2: Box::new(one_specification),
        ..and_specification1
    };
    assert!(and_specification2.is_satisfied_by(&1));
}
