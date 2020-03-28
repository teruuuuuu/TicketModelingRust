use chrono::{Date, Datelike, Local};

use super::specification::Specification;
use super::structs::LocalDate;

pub const MovieDaySpec: MovieDaySpecification = MovieDaySpecification {};

pub struct MovieDaySpecification {}

impl Specification<LocalDate> for MovieDaySpecification {
    fn is_satisfied_by(&self, date: &LocalDate) -> bool {
        date.day() == 1
    }
}

#[test]
fn test_business_date() {
    use super::structs::{DateFromStr, LocalDate};
    let movie_day: LocalDate = LocalDate::date_from_str("2020/03/01");
    let not_movie_day: LocalDate = LocalDate::date_from_str("2020/03/02");

    assert!(MovieDaySpec.is_satisfied_by(&movie_day));
    assert!(!MovieDaySpec.is_satisfied_by(&not_movie_day));
}
