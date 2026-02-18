use std::fmt;
use chrono::{NaiveDate, Datelike};

#[derive(Debug, PartialEq)]
struct MonthDay {
    month: u32,
    day: u32,
}

impl MonthDay {
    fn new(month: u32, day: u32) -> Self {
        Self { month, day }
    }
    fn from_str(s: &str) -> Self {
        assert!(s.len() == 4);
        let month_string = &s[..2];
        let month = month_string.parse().unwrap();
        let day: u32 = s[2..].parse().unwrap();
        MonthDay { month, day }
    }
}

#[derive(Debug)]
struct Category {
    primary: String,
    secondary: Option<String>,
}

impl Category {
    fn new(primary: &str, secondary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: Some(secondary.to_string())
        }
    }
    fn from_primary(primary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: None
        }
    }
    fn from_str(s: &str) -> Category {
        let parts: Vec<&str> = s.split("/").collect();
        
        if parts.len() < 2 {
            Category { primary: parts[0].to_string(), secondary: None }
        } 
        else {
            Category { primary: parts[0].to_string(), secondary: Some(parts[1].to_string())}
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

#[derive(Debug)]
enum EventKind {
    Singular(NaiveDate),
}

#[derive(Debug)]
struct Event {
    kind: EventKind,
    description: String,
    category: Category,
}

impl Event {
    fn new_singular(date: NaiveDate, description: String, category: Category) -> Self {
        Event {
            kind: EventKind::Singular(date),
            description,
            category
        }
    }
    fn year(&self) -> i32 {
        match &self.kind {
            EventKind::Singular(date) => date.year(),
        }
    }
    fn month_day(&self) -> MonthDay {
        match &self.kind {
            EventKind::Singular(date) =>
            MonthDay { month: date.month(), day: date.day() },
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} ({})",
        match &self.kind {
            EventKind::Singular(date) => date.year(),
        },
        self.description, self.category)
    }
}

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

fn main() {
    let mut events: Vec<Event> = Vec::new();
    events.push(Event::new_singular(
        NaiveDate::from_ymd_opt(2025, 12, 11).unwrap(),
        String::from("Rust 1.92.0 released"),
        Category::from_str("programming/rust")
    ));
    
    events.push(Event::new_singular(
        NaiveDate::from_ymd_opt(2015, 5, 15).unwrap(),
        String::from("Rust 1.0.0 released"),
        Category::new("programming", "rust")
    ));
    
    for event in events {
        println!("{}: {}", event.year(), event.description);
    }
}