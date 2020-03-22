use super::specification::Specification;
use super::structs::{Customer, Identification, StudentCard};

pub enum CustomerSpec {
    CinematicCitizen,
    CinemaCitizenSenior,
    General,
    Senior,
    UniversityStudent,
    HighSchoolStudent,
    ElementarySchoolStudent,
}

impl Specification<Customer> for CustomerSpec {
    fn is_satisfied_by(&self, customer: &Customer) -> bool {
        match self {
            CustomerSpec::CinematicCitizen => is_cinema_citizen(customer),
            CustomerSpec::CinemaCitizenSenior => is_cinema_citizen_senior(customer),
            CustomerSpec::Senior => is_senior(customer),
            CustomerSpec::UniversityStudent => is_university_student(customer),
            CustomerSpec::HighSchoolStudent => is_high_school_student(customer),
            CustomerSpec::ElementarySchoolStudent => is_elementary_school_student(customer),
            CustomerSpec::General => is_general(customer),
            _ => false,
        }
    }
}

fn is_cinema_citizen(customer: &Customer) -> bool {
    customer.has_identification(&Identification::MembershipCard)
}

fn is_cinema_citizen_senior(customer: &Customer) -> bool {
    customer.age() >= 60
}

fn is_senior(customer: &Customer) -> bool {
    customer.age() >= 70
}

fn is_university_student(customer: &Customer) -> bool {
    customer.has_identification(&Identification::Student(StudentCard::University))
}

fn is_high_school_student(customer: &Customer) -> bool {
    customer.has_identification(&Identification::Student(StudentCard::HighSchool))
}

fn is_elementary_school_student(customer: &Customer) -> bool {
    customer.has_identification(&Identification::Student(StudentCard::Elementary))
}

fn is_student(customer: &Customer) -> bool {
    is_university_student(customer)
        || is_high_school_student(customer)
        || is_elementary_school_student(customer)
}

fn is_general(customer: &Customer) -> bool {
    !is_cinema_citizen(customer)
        && !is_cinema_citizen_senior(customer)
        && !is_senior(customer)
        && !is_student(customer)
}

#[test]
fn test_customer_spec() {
    use super::structs::{DateFromStr, Gender, LocalDate};

    assert!(CustomerSpec::CinematicCitizen.is_satisfied_by(&Customer {
        birth_day: LocalDate::date_from_str("1987/09/16"),
        gender: Gender::Male,
        identifications: vec![Identification::MembershipCard],
    }));
    assert!(
        !CustomerSpec::CinemaCitizenSenior.is_satisfied_by(&Customer {
            birth_day: LocalDate::date_from_str("1987/09/16"),
            gender: Gender::Male,
            identifications: vec![Identification::MembershipCard],
        })
    );
    assert!(!CustomerSpec::UniversityStudent.is_satisfied_by(&Customer {
        birth_day: LocalDate::date_from_str("1987/09/16"),
        gender: Gender::Male,
        identifications: vec![Identification::MembershipCard],
    }));
    assert!(!CustomerSpec::General.is_satisfied_by(&Customer {
        birth_day: LocalDate::date_from_str("1987/09/16"),
        gender: Gender::Male,
        identifications: vec![Identification::MembershipCard],
    }));

    assert!(CustomerSpec::Senior.is_satisfied_by(&Customer {
        birth_day: LocalDate::date_from_str("1927/09/16"),
        gender: Gender::Male,
        identifications: vec![Identification::MembershipCard],
    }));
    assert!(
        CustomerSpec::CinemaCitizenSenior.is_satisfied_by(&Customer {
            birth_day: LocalDate::date_from_str("1927/09/16"),
            gender: Gender::Male,
            identifications: vec![Identification::MembershipCard],
        })
    );
    assert!(!CustomerSpec::General.is_satisfied_by(&Customer {
        birth_day: LocalDate::date_from_str("1927/09/16"),
        gender: Gender::Male,
        identifications: vec![Identification::MembershipCard],
    }));

    assert!(CustomerSpec::UniversityStudent.is_satisfied_by(&Customer {
        birth_day: LocalDate::date_from_str("1987/09/16"),
        gender: Gender::Male,
        identifications: vec![Identification::Student(StudentCard::University)],
    }));
    assert!(!CustomerSpec::General.is_satisfied_by(&Customer {
        birth_day: LocalDate::date_from_str("1987/09/16"),
        gender: Gender::Male,
        identifications: vec![Identification::Student(StudentCard::University)],
    }));

    assert!(!CustomerSpec::UniversityStudent.is_satisfied_by(&Customer {
        birth_day: LocalDate::date_from_str("1987/09/16"),
        gender: Gender::Male,
        identifications: vec![],
    }));
    assert!(CustomerSpec::General.is_satisfied_by(&Customer {
        birth_day: LocalDate::date_from_str("1987/09/16"),
        gender: Gender::Male,
        identifications: vec![],
    }));
}
