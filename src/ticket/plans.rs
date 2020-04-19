use super::business_day_specification::BusinessDaySpec;
use super::customer_specification::CustomerSpec;
use super::late_specification::{LateSpec8, LateSpecification};
use super::plan_specification::PlanSpecification;
use super::specification::{Spec, Specification};
use super::structs::{Customer, DateFromStr, Gender, Identification, LocalDate, LocalDateTime};
use super::structs::{Plan, PlanCondition, PlanName};

#[macro_use]
use lazy_static::lazy_static;

lazy_static! {
    pub static ref all_plans: Vec<Plan> = {
        fn make_weekday_notlate_plan(
            name: PlanName,
            customer_spec: CustomerSpec,
            price: i32,
        ) -> Plan {
            Plan {
                name: name,
                price: price,
                spec: PlanSpecification {
                    customer_spec: Box::new(customer_spec),
                    business_day_spec_opt: Option::Some(Spec::And(
                        Box::new(Spec::Normal(Box::new(BusinessDaySpec::Weekday))),
                        Box::new(Spec::Not(Box::new(Spec::Normal(Box::new(LateSpec8))))),
                    )),
                    movie_day_spec_opt: Option::None,
                },
            }
        };
        let mut weekday_notlate_plans = vec![
            make_weekday_notlate_plan(
                PlanName::CinemaCitizenSenior,
                CustomerSpec::CinemaCitizenSenior,
                1000,
            ),
            make_weekday_notlate_plan(
                PlanName::CinemaCitizen,
                CustomerSpec::CinematicCitizen,
                1000,
            ),
            make_weekday_notlate_plan(PlanName::General, CustomerSpec::General, 1800),
            make_weekday_notlate_plan(PlanName::Senior, CustomerSpec::Senior, 1100),
            make_weekday_notlate_plan(
                PlanName::UniversityStudent,
                CustomerSpec::UniversityStudent,
                1500,
            ),
            make_weekday_notlate_plan(
                PlanName::HighSchoolStudent,
                CustomerSpec::HighSchoolStudent,
                1000,
            ),
        ];
        fn make_weekday_late_plan(name: PlanName, customer_spec: CustomerSpec, price: i32) -> Plan {
            Plan {
                name: name,
                price: price,
                spec: PlanSpecification {
                    customer_spec: Box::new(customer_spec),
                    business_day_spec_opt: Option::Some(Spec::And(
                        Box::new(Spec::Normal(Box::new(BusinessDaySpec::Weekday))),
                        Box::new(Spec::Normal(Box::new(LateSpec8))),
                    )),
                    movie_day_spec_opt: Option::None,
                },
            }
        };
        let mut weekday_late_plans = vec![
            make_weekday_late_plan(
                PlanName::CinemaCitizen,
                CustomerSpec::CinematicCitizen,
                1000,
            ),
            make_weekday_late_plan(
                PlanName::CinemaCitizenSenior,
                CustomerSpec::CinematicCitizen,
                1000,
            ),
            make_weekday_late_plan(PlanName::Senior, CustomerSpec::Senior, 1100),
            make_weekday_late_plan(PlanName::General, CustomerSpec::General, 1300),
            make_weekday_late_plan(
                PlanName::UniversityStudent,
                CustomerSpec::UniversityStudent,
                1300,
            ),
            make_weekday_late_plan(
                PlanName::HighSchoolStudent,
                CustomerSpec::HighSchoolStudent,
                1000,
            ),
        ];

        fn make_holiday_notlate_plan(
            name: PlanName,
            customer_spec: CustomerSpec,
            price: i32,
        ) -> Plan {
            Plan {
                name: name,
                price: price,
                spec: PlanSpecification {
                    customer_spec: Box::new(customer_spec),
                    business_day_spec_opt: Option::Some(Spec::And(
                        Box::new(Spec::Normal(Box::new(BusinessDaySpec::Holiday))),
                        Box::new(Spec::Not(Box::new(Spec::Normal(Box::new(LateSpec8))))),
                    )),
                    movie_day_spec_opt: Option::None,
                },
            }
        };
        let mut holiday_notlate_plans = vec![
            make_holiday_notlate_plan(
                PlanName::CinemaCitizen,
                CustomerSpec::CinematicCitizen,
                1300,
            ),
            make_holiday_notlate_plan(
                PlanName::CinemaCitizenSenior,
                CustomerSpec::CinemaCitizenSenior,
                1000,
            ),
            make_holiday_notlate_plan(
                PlanName::CinemaCitizenSenior,
                CustomerSpec::CinemaCitizenSenior,
                1100,
            ),
            make_holiday_notlate_plan(PlanName::General, CustomerSpec::General, 1800),
        ];
        let mut all = Vec::new();
        all.append(&mut weekday_notlate_plans);
        all.append(&mut weekday_late_plans);
        all.append(&mut holiday_notlate_plans);

        fn sort(mut plans: Vec<Plan>) -> Vec<Plan> {
            plans.sort_by(|a, b| a.price.cmp(&b.price));
            plans
        }
        sort(all)
    };
}
