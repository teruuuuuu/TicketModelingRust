// use super::business_day_specification::BusinessDaySpec;
// use super::customer_specification::CustomerSpec;
// use super::late_specification::{LateSpec8, LateSpecification};
// use super::movie_day_specification::{MovieDaySpec, MovieDaySpecification};
use super::specification::Specification;
use super::structs::{Customer, LocalDate, LocalDateTime, PlanCondition};

#[derive(Debug)]
pub struct PlanSpecification {
    pub customer_spec: Box<dyn Specification<Customer>>,
    pub business_day_spec_opt: Option<Box<dyn Specification<LocalDateTime>>>,
    pub movie_day_spec_opt: Option<Box<dyn Specification<LocalDate>>>,
}

impl PlanSpecification {
    pub fn new(
        customer_spec: Box<dyn Specification<Customer>>,
        business_day_spec_opt: Option<Box<dyn Specification<LocalDateTime>>>,
        movie_day_spec_opt: Option<Box<dyn Specification<LocalDate>>>,
    ) -> Result<PlanSpecification, &'static str> {
        if business_day_spec_opt.is_some() && movie_day_spec_opt.is_some() {
            Err("PlanSpecification::new Error because both business day and movie day")
        } else if business_day_spec_opt.is_none() && movie_day_spec_opt.is_none() {
            Err("PlanSpecification::new Error because neither business day nor movie day")
        } else {
            Ok(PlanSpecification {
                customer_spec: customer_spec,
                business_day_spec_opt: business_day_spec_opt,
                movie_day_spec_opt: movie_day_spec_opt,
            })
        }
    }
}

impl Specification<PlanCondition> for PlanSpecification {
    fn is_satisfied_by(&self, plan_condition: &PlanCondition) -> bool {
        let customer_spec_result = self.customer_spec.is_satisfied_by(&plan_condition.customer);
        let movie_spec_result = match &self.movie_day_spec_opt {
            Option::None => true,
            Option::Some(movie_day_spec) => {
                movie_day_spec.is_satisfied_by(&plan_condition.local_date_time.date())
            }
        };
        let business_day_spec_result = match &self.business_day_spec_opt {
            Option::None => true,
            Option::Some(business_day_spec) => {
                business_day_spec.is_satisfied_by(&plan_condition.local_date_time)
            }
        };

        customer_spec_result && movie_spec_result && business_day_spec_result
    }
}

#[test]
fn test_plan_spec() {
    use super::customer_specification::CustomerSpec;
    use super::late_specification::LateSpec8;
    use super::movie_day_specification::MovieDaySpec;
    use super::structs::{DateFromStr, Gender, Identification};

    let err_result1 =
        PlanSpecification::new(Box::new(CustomerSpec::General), Option::None, Option::None);
    assert!(err_result1.is_err());

    let err_result2 = PlanSpecification::new(
        Box::new(CustomerSpec::General),
        Option::Some(Box::new(LateSpec8)),
        Option::Some(Box::new(MovieDaySpec)),
    );
    assert!(err_result2.is_err());

    let ok_result1 = PlanSpecification::new(
        Box::new(CustomerSpec::CinematicCitizen),
        Option::Some(Box::new(LateSpec8)),
        Option::None,
    );
    assert!(ok_result1.is_ok());

    let member1 = Customer {
        birth_day: LocalDate::date_from_str("1987/09/16"),
        gender: Gender::Male,
        identifications: vec![Identification::MembershipCard],
    };
    let plan_condition1 = PlanCondition {
        customer: member1,
        local_date_time: LocalDateTime::date_from_str("2020/03/20 21:00:00"),
    };
    assert!(match ok_result1 {
        Result::Err(_) => false,
        Result::Ok(plan_spec) => plan_spec.is_satisfied_by(&plan_condition1),
    });
}
