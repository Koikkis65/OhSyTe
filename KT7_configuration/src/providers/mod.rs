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
        let mut event: Event = Event::new_singular(
            NaiveDate::parse_from_str("2001-01-15", "%Y-%m-%d").unwrap(),
            String::from("Wikipedia julkaistu, Jimmy Wales ja Larry Sangeri"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1920-01-16", "%Y-%m-%d").unwrap(),
            String::from("Ensimmäinen kansainliiton kokous Pariisissa"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1991-01-17", "%Y-%m-%d").unwrap(),
            String::from("Operaatio Desert Storm, Yhdysvallat sekoilee"),
            Category::new("History", "USA"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1919-01-18", "%Y-%m-%d").unwrap(),
            String::from("Pariisin rauhankonferenssi WW1 jälkeen"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1966-01-19", "%Y-%m-%d").unwrap(),
            String::from("Intia sai ensimmäisen naispuolisen pääministerin"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("2009-01-20", "%Y-%m-%d").unwrap(),
            String::from("Obama presidentiksi"),
            Category::new("History", "USA"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1793-01-21", "%Y-%m-%d").unwrap(),
            String::from("Ranskan kuningas Ludvig XVI teloitettiin"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1506-01-22", "%Y-%m-%d").unwrap(),
            String::from("Ensimmäinen sveitsiläiskaartin yksikkö saapuu Vatikaaniin"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1986-01-23", "%Y-%m-%d").unwrap(),
            String::from("Ensimmäiset artistit induktoidaan Rock and Roll Hall of Fameen"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1556-01-23", "%Y-%m-%d").unwrap(),
            String::from("Shaanxin maanjäristys, yksi historian tuhoisimmista"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1848-01-24", "%Y-%m-%d").unwrap(),
            String::from("Kultalöytö Sutter’s Millissä, Kalifornia – kultaryntäys alkaa"),
            Category::new("History", "USA"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1908-01-24", "%Y-%m-%d").unwrap(),
            String::from("Ensimmäinen Boy Scout -trooppi organisoitu Englannissa"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1949-01-25", "%Y-%m-%d").unwrap(),
            String::from("Ensimmäiset Israelin vaalit: David Ben-Gurionin Mapai puolue voittaa"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1971-01-25", "%Y-%m-%d").unwrap(),
            String::from("Idi Aminin sotilasvallankaappaus Ugandassa"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1531-01-26", "%Y-%m-%d").unwrap(),
            String::from("1564 Lisbonissa maanjäristys, ~30 000 kuoli"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1482-01-26", "%Y-%m-%d").unwrap(),
            String::from("Pentateuch, juutalaisen Raamatun ensimmäinen painettu painos"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1945-01-27", "%Y-%m-%d").unwrap(),
            String::from("Auschwitz-Birkenaun keskitysleirit vapautetaan toisen maailmansodan aikana"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1916-01-27", "%Y-%m-%d").unwrap(),
            String::from("Britannian sotapalveluslaki ottaa käyttöön asevelvollisuuden"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1986-01-28", "%Y-%m-%d").unwrap(),
            String::from("Space Shuttle Challenger räjähtää laukaisun jälkeen"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1861-01-29", "%Y-%m-%d").unwrap(),
            String::from("Kansas liittyy Yhdysvaltain osavaltioksi"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1886-01-29", "%Y-%m-%d").unwrap(),
            String::from("Karl Benz patentoi ensimmäisen modernin auton"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1971-01-29", "%Y-%m-%d").unwrap(),
            String::from("Viimeinen UFO-havainto Pudasjärvellä, Suomi"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1950-01-23", "%Y-%m-%d").unwrap(),
            String::from("testi listan ulkopuolelta 2"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1950-01-14", "%Y-%m-%d").unwrap(),
            String::from("testi listan ulkopuolelta 3"),
            Category::from_primary("History"),
        );
        events.push(event);
        event = Event::new_singular(
            NaiveDate::parse_from_str("1943-01-18", "%Y-%m-%d").unwrap(),
            String::from("Leningradin piiritys murrettu"),
            Category::new("History", "Russia"),
        );
        events.push(event);
    }
}