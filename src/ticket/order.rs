use super::plans::sort_plans;
use super::structs::{Plan, PlanCondition};

pub fn order_price(plan_condition: PlanCondition) -> Option<i32> {
    order(plan_condition).map(|plan| plan.price)
}

pub fn order(plan_condition: PlanCondition) -> Option<Plan> {
    sort_plans()
        .into_iter()
        .find(|plan| plan.is_satisfied_by(&plan_condition))
}

#[test]
fn test_plans() {
    use super::structs::{Customer, DateFromStr, Gender, Identification, LocalDate, LocalDateTime};

    let plan_condition = PlanCondition {
        customer: Customer {
            birth_day: LocalDate::date_from_str("1987/09/16"),
            gender: Gender::Male,
            identifications: vec![Identification::MembershipCard],
        },
        local_date_time: LocalDateTime::date_from_str("2020/03/20 13:00:00"),
    };
    assert_eq!(1000, order_price(plan_condition).unwrap());
}
