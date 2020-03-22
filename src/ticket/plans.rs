use super::business_day_specification::BusinessDaySpec;
use super::customer_specification::CustomerSpec;
use super::late_specification::{LateSpec8, LateSpecification};
use super::plan_specification::PlanSpecification;
use super::specification::{AndSpecification, NotSpecification, OrSpecification, Specification};
use super::structs::{Customer, DateFromStr, Gender, Identification, LocalDate, LocalDateTime};
use super::structs::{Plan, PlanCondition, PlanName};

fn plans() -> Vec<Plan> {
    let weekday_notlate_plans = vec![
        Plan {
            name: PlanName::CinemaCitizenSenior,
            price: 1000,
            spec: PlanSpecification {
                customer_spec: Box::new(CustomerSpec::CinemaCitizenSenior),
                business_day_spec_opt: Option::Some(Box::new(AndSpecification {
                    spec1: Box::new(BusinessDaySpec::Weekday),
                    spec2: Box::new(NotSpecification {
                        spec: Box::new(LateSpec8),
                    }),
                })),
                movie_day_spec_opt: Option::None,
            },
        },
        Plan {
            name: PlanName::CinemaCitizen,
            price: 1000,
            spec: PlanSpecification {
                customer_spec: Box::new(CustomerSpec::CinematicCitizen),
                business_day_spec_opt: Option::Some(Box::new(AndSpecification {
                    spec1: Box::new(BusinessDaySpec::Weekday),
                    spec2: Box::new(NotSpecification {
                        spec: Box::new(LateSpec8),
                    }),
                })),
                movie_day_spec_opt: Option::None,
            },
        },
        Plan {
            name: PlanName::General,
            price: 1800,
            spec: PlanSpecification {
                customer_spec: Box::new(CustomerSpec::General),
                business_day_spec_opt: Option::Some(Box::new(AndSpecification {
                    spec1: Box::new(BusinessDaySpec::Weekday),
                    spec2: Box::new(NotSpecification {
                        spec: Box::new(LateSpec8),
                    }),
                })),
                movie_day_spec_opt: Option::None,
            },
        },
        Plan {
            name: PlanName::General,
            price: 1800,
            spec: PlanSpecification {
                customer_spec: Box::new(CustomerSpec::General),
                business_day_spec_opt: Option::Some(Box::new(AndSpecification {
                    spec1: Box::new(BusinessDaySpec::Weekday),
                    spec2: Box::new(NotSpecification {
                        spec: Box::new(LateSpec8),
                    }),
                })),
                movie_day_spec_opt: Option::None,
            },
        },
    ];
    weekday_notlate_plans
}

fn sort(mut plans: Vec<Plan>) -> Vec<Plan> {
    plans.sort_by(|a, b| a.price.cmp(&b.price));
    plans
}

pub fn sort_plans() -> Vec<Plan> {
    sort(plans())
}
