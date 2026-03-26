#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

use chrono::{Datelike, NaiveDate};
use std::fmt;

use crate::providers::{EventProvider, SimpleProvider};

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
    day: u8,
}

#[derive(Debug)]
enum EventKind {
    Singular(NaiveDate),
}

#[derive(Debug)]
pub struct Event {
    kind: EventKind,
    pub description: String,
    pub category: Category,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct MonthDay {
    month: u32,
    day: u32,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Category {
    pub primary: String,
    pub secondary: Option<String>,
}

// IMPLEMENTATIONS
impl Date {
    pub fn new(year: u16, month: Month, day: u8) -> Self {
        Self { year, month, day }
    }
}

impl Event {
    pub fn new_singular(date: NaiveDate, description: String, category: Category) -> Self {
        Event {
            kind: EventKind::Singular(date),
            description,
            category,
        }
    }
    pub fn year(&self) -> i32 {
        match &self.kind {
            EventKind::Singular(date) => date.year(),
        }
    }
    pub fn month_day(&self) -> MonthDay {
        match &self.kind {
            EventKind::Singular(date) => MonthDay {
                month: date.month(),
                day: date.day(),
            },
        }
    }
    pub fn category(&self) -> Category {
        self.category.clone()
    }
    pub fn description(&self) -> String {
        self.description.clone()
    }
}

impl MonthDay {
    pub fn new(day: u32, month: u32) -> Self {
        Self { day, month }
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
    fn from_str(s: &str) -> Category {
        let parts: Vec<&str> = s.split("/").collect();

        if parts.len() < 2 {
            Category {
                primary: parts[0].to_string(),
                secondary: None,
            }
        } else {
            Category {
                primary: parts[0].to_string(),
                secondary: Some(parts[1].to_string()),
            }
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.secondary {
            Some(sec) => write!(f, "{}/{}", self.primary, sec),
            None => write!(f, "{}", self.primary),
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {} ({})",
            self.year(),
            self.description,
            self.category
        )
    }
}

/*
// Yksikkötestit category-tietotyypille
#[cfg(test)]
mod tests {
    use crate::Category;
    #[test]
    fn test_category_new() {
        let cat = Category::new("programming", "rust");
        assert_eq!(cat.primary, "programming");
        assert_eq!(cat.secondary, Some("rust".to_string()));
    }
    #[test]
    fn test_category_from_primary() {
        let cat = Category::from_primary("programming");
        assert_eq!(cat.primary, "programming");
        assert_eq!(cat.secondary, None);
    }
    #[test]
    fn test_category_from_str() {
        let cat1 = Category::from_str("programming/rust");
        assert_eq!(cat1.primary, "programming");
        assert_eq!(cat1.secondary, Some("rust".to_string()));
    }
}
*/

// FUNCTIONS
// PUBLIC
pub fn get_events_between_dates(start: MonthDay, end: MonthDay, events: &Vec<Event>) {
    for day in start.day..=end.day {
        let mut has_header: bool = false;
        let this_day: MonthDay = MonthDay::new(day, start.month);

        for event in events {
            if event.month_day() == this_day {
                if !has_header {
                    println!("{}.{:#?} events", event.year(), event.month_day().month);
                    has_header = true;
                }
                if event.category.secondary == None {
                    println!(
                        "year: {} {} categories: {}",
                        event.year(),
                        event.description,
                        event.category.primary
                    );
                } else {
                    println!(
                        "year: {} {} categories: {}, {:#?}",
                        event.year(),
                        event.description,
                        event.category.primary,
                        event.category.secondary.as_ref().unwrap().to_string()
                    );
                }
            }
        }
        if !has_header {
            println!("No events in {}.{:#?}\n", this_day.day, this_day.month)
        } else {
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
        Month::February => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 31,
    }
}
