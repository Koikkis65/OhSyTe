#[derive(Debug, PartialEq, Copy, Clone)]
enum Month {
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

struct Date {
    year: u16,
    month: Month,
    day: u8
}

impl Date {
    fn new(year: u16, month: Month, day: u8) -> Self {
        Self { year, month, day }
    }
}

struct Event {
    date: Date,
    desc: String,
    category: Category,
}

impl Event {
    fn new(date: Date, desc: String, category: Category) -> Self {
        Self { date, desc, category }
    }
    fn month_day(&self) -> MonthDay {
        return MonthDay { month: self.date.month, day: self.date.day }
    }
}

#[derive(Debug, PartialEq)]
struct MonthDay {
    month: Month,
    day: u8,
}

impl MonthDay {
    fn new(day: u8, month: Month) -> Self {
        Self {day, month}
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
            secondary: Some(secondary.to_string()),
        }
    }
    fn from_primary(primary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: None,
        }
    }
}

/*
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
*/

fn main() { 
    
    let events: [Event; 25] = [
        // existing
        Event::new(
            Date::new(2001, Month::January, 15),
            String::from("Wikipedia julkaistu, Jimmy Wales ja Larry Sangeri"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(1920, Month::January, 16),
            String::from("Ensimmäinen kansainliiton kokous Pariisissa"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(1991, Month::January, 17),
            String::from("Operaatio Desert Storm, Yhdysvallat sekoilee"),
            Category::new("History", "USA"),
        ),
        Event::new(
            Date::new(1919, Month::January, 18),
            String::from("Pariisin rauhankonferenssi WW1 jälkeen"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(1966, Month::January, 19),
            String::from("Intia sai ensimmäisen naispuolisen pääministerin"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(2009, Month::January, 20),
            String::from("Obama presidentiksi"),
            Category::new("History", "USA"),
        ),
        Event::new(
            Date::new(1793, Month::January, 21),
            String::from("Ranskan kuningas Ludvig XVI teloitettiin"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(1506, Month::January, 22),
            String::from("Ensimmäinen sveitsiläiskaartin yksikkö saapuu Vatikaaniin"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(1986, Month::January, 23),
            String::from("Ensimmäiset artistit induktoidaan Rock and Roll Hall of Fameen"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(1556, Month::January, 23),
            String::from("Shaanxin maanjäristys, yksi historian tuhoisimmista"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(1848, Month::January, 24),
            String::from("Kultalöytö Sutter’s Millissä, Kalifornia – kultaryntäys alkaa"),
            Category::new("History", "USA"),

        ),
        Event::new(
            Date::new(1908, Month::January, 24),
            String::from("Ensimmäinen Boy Scout -trooppi organisoitu Englannissa"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(1949, Month::January, 25),
            String::from("Ensimmäiset Israelin vaalit: David Ben-Gurionin Mapai puolue voittaa"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(1971, Month::January, 25),
            String::from("Idi Aminin sotilasvallankaappaus Ugandassa"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(1531, Month::January, 26),
            String::from("1564 Lisbonissa maanjäristys, ~30 000 kuoli"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(1482, Month::January, 26),
            String::from("Pentateuch, juutalaisen Raamatun ensimmäinen painettu painos"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(1945, Month::January, 27),
            String::from("Auschwitz-Birkenaun keskitysleirit vapautetaan toisen maailmansodan aikana"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(1916, Month::January, 27),
            String::from("Britannian sotapalveluslaki ottaa käyttöön asevelvollisuuden"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(1986, Month::January, 28),
            String::from("Space Shuttle Challenger räjähtää laukaisun jälkeen"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(1861, Month::January, 29),
            String::from("Kansas liittyy Yhdysvaltain osavaltioksi"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(1886, Month::January, 29),
            String::from("Karl Benz patentoi ensimmäisen modernin auton"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(1971, Month::January, 29),
            String::from("Viimeinen UFO-havainto Pudasjärvellä, Suomi"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(1950, Month::January, 23),
            String::from("testi listan ulkopuolelta 2"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(1950, Month::January, 14),
            String::from("testi listan ulkopuolelta 3"),
            Category::from_primary("History"),
        ),
        Event::new(
            Date::new(1943, Month::January, 18),
            String::from("Leningradin piiritys murrettu"),
            Category::new("History", "Russia"),
        ),

    ];
    
    /*
    Muuta aiempaa ohjelmaasi / tee uusi ohjelma siten, että se poimii tapahtumien joukosta ne, 
    jotka osuvat aikavälille 15.1.-29.1. minä vuonna tahansa. 
    Jos teit tehtävän 2, ota sieltä aikavälin 15.-22.1. tapahtumat, 
    ja lisää tätä tehtävää varten tapahtumia myös välille 23.1.-29.1.
    */
    let start: MonthDay = MonthDay::new(15, Month::January);
    let end: MonthDay = MonthDay::new(29, Month::January);
    get_events_between_dates(start, end, &events);
    
}

fn get_events_between_dates(start: MonthDay, end: MonthDay, events: &[Event; 25]) {
    for day in start.day..=end.day {
        let mut has_header: bool = false;
        let this_day: MonthDay = MonthDay::new(day, start.month);

        for event in events {
            if event.month_day() == this_day {
                if !has_header {
                    println!("{}. {:#?} events", event.date.day, event.date.month);
                    has_header = true;
                }
                if event.category.secondary == None {
                    println!("{} {} categories: {}", event.date.year, event.desc, event.category.primary);
                }
                else {
                    println!("{} {} categories: {}, {:#?}", event.date.year, event.desc, event.category.primary, event.category.secondary.as_ref().unwrap().to_string());
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
