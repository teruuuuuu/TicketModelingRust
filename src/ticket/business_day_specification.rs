use chrono::{Datelike, Weekday};

use super::specification::Specification;
use super::structs::{LocalDate, LocalDateTime};

#[warn(dead_code)]
pub enum BusinessDaySpec {
    Weekday,
    Holiday,
}
impl Specification<LocalDateTime> for BusinessDaySpec {
    fn is_satisfied_by(&self, date_time: &LocalDateTime) -> bool {
        match self {
            BusinessDaySpec::Holiday => !is_business_date_time(date_time),
            BusinessDaySpec::Weekday => is_business_date_time(date_time),
        }
    }
}

fn is_business_date_time(local_date: &LocalDateTime) -> bool {
    is_business_date(&local_date.date())
}

fn is_business_date(local_date: &LocalDate) -> bool {
    match local_date.weekday() {
        Weekday::Sun => false,
        Weekday::Sat => false,
        _ => true,
    }
}

#[test]
fn test_business_date() {
    use super::structs::DateFromStr;

    let sunday = LocalDateTime::date_from_str("2020/03/22 19:00:00");
    let monday = LocalDateTime::date_from_str("2020/03/23 19:00:00");

    assert!(!BusinessDaySpec::Weekday.is_satisfied_by(&sunday));
    assert!(BusinessDaySpec::Weekday.is_satisfied_by(&monday));
    assert!(BusinessDaySpec::Holiday.is_satisfied_by(&sunday));
    assert!(!BusinessDaySpec::Holiday.is_satisfied_by(&monday));
}
