use super::specification::Specification;
use super::structs::LocalDateTime;

use chrono::Timelike;

pub const LateSpec8: LateSpecification = LateSpecification { start_hour: 20 };

pub struct LateSpecification {
    pub start_hour: i32,
}

impl Specification<LocalDateTime> for LateSpecification {
    fn is_satisfied_by(&self, local_date_time: &LocalDateTime) -> bool {
        self.start_hour <= local_date_time.hour() as i32
    }
}

#[test]
fn test_late_spec() {
    use super::structs::DateFromStr;

    let late_time = LocalDateTime::date_from_str("2020/03/20 21:00:00");
    let not_late_time = LocalDateTime::date_from_str("2020/03/20 15:00:00");

    assert!(LateSpec8.is_satisfied_by(&late_time));
    assert!(!LateSpec8.is_satisfied_by(&not_late_time));
}
