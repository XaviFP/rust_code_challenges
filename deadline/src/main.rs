use chrono::{Local, NaiveDate};

fn main() {
    println!("Hello, world!");
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

struct ImportantEvent {
    when: NaiveDate,
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        self.when < Local::now().naive_local().date()
    }
}

#[test]
fn past_event() {
    let ev = ImportantEvent{when: NaiveDate::from_ymd_opt(2020, 4, 13).unwrap()};//DateTime::parse_from_str("2021 Dec 15", "%Y %b %d").unwrap()};
    assert_eq!(ev.is_passed(), true)
}

#[test]
fn future_event() {
    let ev = ImportantEvent{when: NaiveDate::from_ymd_opt(2025, 12, 25).unwrap()};//DateTime::parse_from_str("2025 Jan 31", "%Y %b %d").unwrap()};
    assert_eq!(ev.is_passed(), false)
}
