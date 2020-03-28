use super::plans::sort_plans;
use super::structs::{Plan, PlanCondition};

pub struct AllPlans {
    pub sorted_plans: Vec<Plan>,
}

impl AllPlans {
    pub fn new() -> AllPlans {
        AllPlans {
            sorted_plans: sort_plans(),
        }
    }

    pub fn order(&self, condition: &PlanCondition) -> Option<&Plan> {
        self.sorted_plans
            .iter()
            .find(|&plan| plan.is_satisfied_by(&condition))
    }

    pub fn order_price(&self, condition: &PlanCondition) -> Option<i32> {
        match self.order(condition) {
            Option::Some(plan) => Option::Some(plan.price),
            Option::None => Option::None,
        }
    }
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

    let all_plans = AllPlans::new();
    assert_eq!(Option::Some(1000), all_plans.order_price(&plan_condition));
}
