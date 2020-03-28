extern crate ticket_modeling_rust;

use ticket_modeling_rust::order::AllPlans;
use ticket_modeling_rust::structs::*;

fn main() {
    let plan_condition = PlanCondition {
        customer: Customer {
            birth_day: LocalDate::date_from_str("1987/09/16"),
            gender: Gender::Male,
            identifications: vec![Identification::MembershipCard],
        },
        local_date_time: LocalDateTime::date_from_str("2020/03/20 13:00:00"),
    };

    let all_plans = AllPlans::new();
    println!("{:?}", all_plans.order(&plan_condition));
}
