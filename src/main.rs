extern crate ticket_modeling_rust;

use ticket_modeling_rust::order::order;
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
    println!("{:?}", order(plan_condition));
}
