use super::plan_specification::PlanSpecification;
use super::specification::Specification;
use chrono::{Date, DateTime, Datelike, Local, TimeZone};

pub type LocalDate = Date<Local>;
pub type LocalDateTime = DateTime<Local>;

#[derive(Debug)]
pub struct Plan {
    pub name: PlanName,
    pub price: i32,
    pub spec: PlanSpecification,
}
impl Plan {
    pub fn is_satisfied_by(&self, plan_condition: &PlanCondition) -> bool {
        self.spec.is_satisfied_by(&plan_condition)
    }
}

#[derive(Debug, Clone)]
pub enum PlanName {
    CinemaCitizen,
    CinemaCitizenSenior,
    General,
    Senior,
    UniversityStudent,
    HighSchoolStudent,
    ElementaryStudent,
}

impl PlanName {
    pub fn to_string(&self) -> &str {
        match self {
            PlanName::CinemaCitizen => "シネマシティズン",
            PlanName::CinemaCitizenSenior => "シネマシティズン(60才以上)",
            PlanName::General => "一般",
            PlanName::Senior => "シニア(70才以上)",
            PlanName::UniversityStudent => "学生(大・専)",
            PlanName::HighSchoolStudent => "中・高校生",
            PlanName::ElementaryStudent => "小学生",
            _ => "",
        }
    }
}

#[derive(Debug)]
pub struct PlanCondition {
    pub customer: Customer,
    pub local_date_time: LocalDateTime,
}

#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StudentCard {
    University,
    HighSchool,
    Elementary,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Identification {
    MembershipCard,
    DisabilityHandbook,
    Student(StudentCard),
}

#[derive(Debug, Clone)]
pub struct Customer {
    pub birth_day: LocalDate,
    pub gender: Gender,
    pub identifications: Vec<Identification>,
}

impl Customer {
    pub fn has_identification(&self, identification: &Identification) -> bool {
        self.identifications.contains(&identification)
    }

    pub fn age(&self) -> i32 {
        let now_date = Local::now().naive_local();
        match (now_date.month() > self.birth_day.month())
            || (now_date.month() == self.birth_day.month()
                && now_date.day() >= self.birth_day.day())
        {
            true => now_date.year() - self.birth_day.naive_local().year(),
            false => now_date.year() - self.birth_day.naive_local().year() - 1,
        }
    }
}

pub trait DateFromStr<T> {
    fn date_from_str(date_str: &str) -> T;
}
impl DateFromStr<LocalDateTime> for LocalDateTime {
    fn date_from_str(date_str: &str) -> LocalDateTime {
        Local
            .datetime_from_str(date_str, "%Y/%m/%d %H:%M:%S")
            .unwrap()
    }
}
impl DateFromStr<LocalDate> for LocalDate {
    fn date_from_str(date_str: &str) -> LocalDate {
        LocalDateTime::date_from_str(&format!("{} 00:00:00", date_str)).date()
    }
}

#[test]
fn test_plan_structs() {
    let customer = Customer {
        birth_day: LocalDate::date_from_str("1987/09/16"),
        gender: Gender::Male,
        identifications: vec![Identification::MembershipCard],
    };

    assert!(customer.has_identification(&Identification::MembershipCard));
    assert!(!customer.has_identification(&Identification::Student(StudentCard::Elementary)));
}
