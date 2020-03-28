#[derive(Debug)]
pub enum Spec<T> {
    Normal(Box<dyn Specification<T>>),
    And(Box<Spec<T>>, Box<Spec<T>>),
    Or(Box<Spec<T>>, Box<Spec<T>>),
    Not(Box<Spec<T>>),
}

impl<T> Specification<T> for Spec<T> {
    fn is_satisfied_by(&self, arg: &T) -> bool {
        match self {
            Spec::Normal(spec) => spec.is_satisfied_by(arg),
            Spec::And(spec1, spec2) => spec1.is_satisfied_by(arg) && spec2.is_satisfied_by(arg),
            Spec::Or(spec1, spec2) => spec1.is_satisfied_by(arg) || spec2.is_satisfied_by(arg),
            Spec::Not(spec) => !spec.is_satisfied_by(arg),
        }
    }
}

pub trait Specification<T> {
    fn is_satisfied_by(&self, arg: &T) -> bool;
}

impl<T: ?Sized> std::fmt::Debug for dyn Specification<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
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

    assert!(1.is_satisfied_by(&1));
    assert!(!1.is_satisfied_by(&2));

    let or_specification = Spec::Or(
        Box::new(Spec::Normal(Box::new(1))),
        Box::new(Spec::Normal(Box::new(2))),
    );
    assert!(or_specification.is_satisfied_by(&1));
    assert!(!or_specification.is_satisfied_by(&3));

    let and_specification1 = Spec::And(
        Box::new(Spec::Normal(Box::new(1))),
        Box::new(Spec::Normal(Box::new(2))),
    );
    assert!(!and_specification1.is_satisfied_by(&1));

    let and_specification2 = Spec::And(
        Box::new(Spec::Normal(Box::new(1))),
        Box::new(Spec::Not(Box::new(Spec::Normal(Box::new(2))))),
    );
    assert!(and_specification2.is_satisfied_by(&1));
}
