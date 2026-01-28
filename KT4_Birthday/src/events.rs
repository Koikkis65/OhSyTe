#![allow(
    unused_variables,
    unused_imports,
    dead_code
)]

// pub
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

pub struct Date {
    year: u16,
    month: Month,
    day: u8
}

pub struct Event {
    date: Date,
    desc: String,
    category: Category,
}

#[derive(Debug, PartialEq)]
pub struct MonthDay {
    month: Month,
    day: u8,
}

#[derive(Debug)]
pub struct Category {
    primary: String,
    secondary: Option<String>,
}

// IMPLEMENTATIONS
impl Date {
    pub fn new(year: u16, month: Month, day: u8) -> Self {
        Self { year, month, day }
    }
}

impl Event {
    pub fn new(date: Date, desc: String, category: Category) -> Self {
        Self { date, desc, category }
    }
    fn month_day(&self) -> MonthDay {
        return MonthDay { month: self.date.month, day: self.date.day }
    }
}

impl MonthDay {
    pub fn new(day: u8, month: Month) -> Self {
        Self {day, month}
    }
}

impl Category {
    pub fn new(primary: &str, secondary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: Some(secondary.to_string()),
        }
    }
    pub fn from_primary(primary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: None,
        }
    }
}

// FUNCTIONS
// PUBLIC
pub fn get_events_between_dates(start: MonthDay, end: MonthDay, events: &Vec<Event>) {
    for day in start.day..=end.day {
        let mut has_header: bool = false;
        let this_day: MonthDay = MonthDay::new(day, start.month);

        for event in events {
            if event.month_day() == this_day {
                if !has_header {
                    println!("{}.{:#?} events", event.date.day, event.date.month);
                    has_header = true;
                }
                if event.category.secondary == None {
                    println!("year: {} {} categories: {}", event.date.year, event.desc, event.category.primary);
                }
                else {
                    println!("year: {} {} categories: {}, {:#?}", event.date.year, event.desc, event.category.primary, event.category.secondary.as_ref().unwrap().to_string());
                }
            }
        }
        if !has_header {
            println!("No events in {}.{:#?}\n", this_day.day, this_day.month)
        }
        else {
            println!();
        }
    }
}

// PRIVATE
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn day_count(month: Month, year: i32) -> u8 {
    match month {
        Month::April | Month::June | Month::September | Month::November => 30,
        Month::February => if is_leap_year(year) { 29 } else { 28 },
        _ => 31
    }
}