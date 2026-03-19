#![allow(dead_code, unused_variables, unused_imports, unused_mut)]
pub mod my_provider;
pub mod sqlite;
use chrono::{NaiveDate, Local};

use crate::events::{Event, Category};
pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, events: &mut Vec<Event>);
}

pub struct SimpleProvider {
    name: String,
}

impl SimpleProvider {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

impl EventProvider for SimpleProvider {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn get_events(&self, events: &mut Vec<Event>) {
        events.push(Event::new_singular(
            NaiveDate::parse_from_str("1991-01-17", "%Y-%m-%d").unwrap(), 
            String::from("Operaatio Desert Storm, Yhdysvallat sekoilee"), 
            Category::from_primary("History")
        ));
        events.push(Event::new_singular(
            NaiveDate::parse_from_str("1995-01-18", "%Y-%m-%d").unwrap(), 
            String::from("Suomi liittyy Euroopan unioniin"), 
            Category::from_primary("History")
        ));
    }
}