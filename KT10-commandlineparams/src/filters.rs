use crate::events::{Category, Event, MonthDay};
use std::collections::HashSet;


/*
All tests:
accepts anything
accepts text and category
accepts month day and category
accepts text and month day
accepts singular requests of all 3x tests
accepts none of the above. 6x tests

Should be 13 tests total
*/

#[cfg(test)]
mod tests {
    use super::*;
    use KT9_eventFiltering::events::Month;
    use chrono::{Datelike, Local, NaiveDate};

    #[test]
    fn filter_accepts_anything() {
        let rust_category = Category::new("programming", "rust");
        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 5).unwrap(),
            "Rust 1.94.0 released".to_string(),
            rust_category.clone(),
        );
        let filter = FilterBuilder::new().build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn filter_accepts_text_category() {
        let rust_category = Category::new("programming", "rust");
        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 5).unwrap(),
            "Rust 1.94.0 released".to_string(),
            rust_category.clone(),
        );
        let filter = FilterBuilder::new()
            .text("Rust".to_string())
            .category(rust_category)
            .build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn filter_accepts_month_day_category() {
        let md: MonthDay = MonthDay::new(5, 3);
        let rust_category = Category::new("programming", "rust");
        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 5).unwrap(),
            "Rust 1.94.0 released".to_string(),
            rust_category.clone(),
        );
        let filter = FilterBuilder::new()
            .month_day(md)
            .category(rust_category)
            .build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn filter_accepts_month_day_text() {
        let md: MonthDay = MonthDay::new(5, 3);
        let rust_category = Category::new("programming", "rust");
        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 5).unwrap(),
            "Rust 1.94.0 released".to_string(),
            rust_category.clone(),
        );
        let filter = FilterBuilder::new()
            .month_day(md)
            .text("Rust".to_string())
            .build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn filter_accepts_category() {
        let rust_category = Category::new("programming", "rust");
        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 5).unwrap(),
            "Rust 1.94.0 released".to_string(),
            rust_category.clone(),
        );
        let filter = FilterBuilder::new()
            .category(rust_category)
            .build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn filter_accepts_description() {
        let rust_category = Category::new("programming", "rust");
        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 5).unwrap(),
            "Rust 1.94.0 released".to_string(),
            rust_category.clone(),
        );
        let filter = FilterBuilder::new()
            .text("released".to_string())
            .build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn filter_accepts_month_day() {
        let md: MonthDay = MonthDay::new(5, 3);
        let rust_category = Category::new("programming", "rust");
        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 5).unwrap(),
            "Rust 1.94.0 released".to_string(),
            rust_category.clone(),
        );
        let filter = FilterBuilder::new()
            .month_day(md)
            .build();
        assert!(filter.accepts(&event));
    }

    #[test]
    fn filter_fails_category() {
        let md: MonthDay = MonthDay::new(5, 3);
        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 5).unwrap(),
            "Rust 1.94.0 released".to_string(),
            Category::new("programming", "rust"),
        );
        let filter = FilterBuilder::new()
            .category(Category::new("whats", "up"))
            .build();
        assert!(!filter.accepts(&event));
    }

    #[test]
    fn filter_fails_description() {
        let md: MonthDay = MonthDay::new(5, 3);
        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 5).unwrap(),
            "Rust 1.94.0 released".to_string(),
            Category::new("programming", "rust"),
        );
        let filter = FilterBuilder::new()
            .text("Java".to_string())
            .build();
        assert!(!filter.accepts(&event));
    }

    #[test]
    fn filter_fails_month_day() {
        let md: MonthDay = MonthDay::new(6, 4);
        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 5).unwrap(),
            "Rust 1.94.0 released".to_string(),
            Category::new("programming", "rust"),
        );
        let filter = FilterBuilder::new()
            .month_day(md)
            .build();
        assert!(!filter.accepts(&event));
    }

    #[test]
    fn filter_fails_text_category() {
        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 5).unwrap(),
            "Rust 1.94.0 released".to_string(),
            Category::new("programming", "rust"),
        );
        let filter = FilterBuilder::new()
            .text("Rust".to_string())
            .category(Category::new("Whats", "up"))
            .build();
        assert!(!filter.accepts(&event));
    }

    #[test]
    fn filter_fails_month_day_category() {
        let md: MonthDay = MonthDay::new(6, 3);
        let rust_category = Category::new("programming", "rust");
        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 5).unwrap(),
            "Rust 1.94.0 released".to_string(),
            rust_category.clone(),
        );
        let filter = FilterBuilder::new()
            .month_day(md)
            .category(rust_category)
            .build();
        assert!(!filter.accepts(&event));
    }

    #[test]
    fn filter_fails_month_day_text() {
        let md: MonthDay = MonthDay::new(5, 3);
        let rust_category = Category::new("programming", "rust");
        let event = Event::new_singular(
            NaiveDate::from_ymd_opt(2026, 3, 5).unwrap(),
            "Rust 1.94.0 released".to_string(),
            rust_category.clone(),
        );
        let filter = FilterBuilder::new()
            .month_day(md)
            .text("python".to_string())
            .build();
        assert!(!filter.accepts(&event));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FilterOption {
    MonthDay(MonthDay),
    Category(Category),
    Text(String),
}

pub struct EventFilter {
    options: HashSet<FilterOption>,
}

pub struct FilterBuilder {
    options: HashSet<FilterOption>,
}

impl EventFilter {
    pub fn new() -> Self {
        Self {
            options: HashSet::new(),
        }
    }
    pub fn accepts(&self, event: &Event) -> bool {
        // If the option set is empty, this is an all-pass filter.
        if self.options.is_empty() {
            return true;
        }
        // Collect the results from various options into a vector.
        let mut results: Vec<bool> = Vec::new();
        for option in self.options.iter() {
            let result = match option {
                FilterOption::MonthDay(month_day) => *month_day == event.month_day(),
                FilterOption::Category(category) => *category == event.category(),
                FilterOption::Text(text) => event.description().contains(text),
            };
            results.push(result);
        }
        // If the results vector contains only `true values,
        // all the options match, and the event will be accepted,
        // otherwise it will be rejected by the filter.
        results.iter().all(|&option| option)
    }
}

impl FilterBuilder {
    pub fn new() -> FilterBuilder {
        FilterBuilder {
            options: HashSet::new(),
        }
    }
    pub fn month_day(mut self, month_day: MonthDay) -> FilterBuilder {
        self.options.insert(FilterOption::MonthDay(month_day));
        self
    }
    pub fn category(mut self, category: Category) -> FilterBuilder {
        self.options.insert(FilterOption::Category(category));
        self
    }
    pub fn text(mut self, text: String) -> FilterBuilder {
        self.options.insert(FilterOption::Text(text));
        self
    }
    pub fn build(self) -> EventFilter {
        EventFilter {
            options: self.options,
        }
    }
}
