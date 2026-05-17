mod events;
mod birthday;
mod providers;

use chrono::{Date, NaiveDate};

use crate::providers::{EventProvider, SimpleProvider};
use crate::events::{Event, Category};

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::providers::{EventProvider, SimpleProvider};
    use crate::events::{Event, Category};
    #[test]
    fn test_provider() {
        let mut events: Vec<Event> = vec![];
        let provider: SimpleProvider = SimpleProvider::new("SimpleProvider");
        provider.get_events(&mut events);
        assert_eq!(events.len(), 25);
    }

    #[test]
    fn test_provider_item_3() {
        let mut events: Vec<Event> = vec![];
        let provider: SimpleProvider = SimpleProvider::new("SimpleProvider");
        provider.get_events(&mut events);
        assert_eq!(events[2].description, "Operaatio Desert Storm, Yhdysvallat sekoilee");
    }

    #[test]
    fn test_provider_item_20_category() {
        let mut events: Vec<Event> = vec![];
        let provider: SimpleProvider = SimpleProvider::new("SimpleProvider");
        provider.get_events(&mut events);
        assert_eq!(events[19].category.primary, "History");
        assert_eq!(events[19].category.secondary, None);
    }
}

fn main() {
    /*
    All in all I have to admit the get_events() function is confusing and our instructions were not clear.
    I implemented it so that the provider fills the events vector with events, but I am not sure if that is what was intended.
     */
    let mut events: Vec<Event> = vec![];
    let provider: SimpleProvider = SimpleProvider::new("SimpleProvider");
    provider.get_events(&mut events);
}
