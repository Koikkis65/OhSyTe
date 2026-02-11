// mod lib;
mod events;

use std::env;
use events::Category;
use events::Month;
use events::MonthDay;
use events::Date;
use events::Event;

fn main() {
    let args: Vec<String> = env::args().collect();
    let events = vec![
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
            Category::from_primary("History"),
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
            Category::from_primary("History"),
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
            Category::from_primary("History"),
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
            Category::new("History", "General"),
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
    events::get_events_between_dates(start, end, &events);
    
}
