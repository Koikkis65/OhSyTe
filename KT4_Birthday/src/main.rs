#![allow(
    unused_variables,
    unused_imports,
    dead_code
)]

// mod lib;
mod events;
mod birthday;

use std::collections;
use std::env;
use std::env::Args;
use std::vec;
use events::Category;
use events::Month;
use events::MonthDay;
use events::Date;
use events::Event;
use chrono::prelude::*;
use birthday::*;

fn main() {
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

    let args: Vec<String> = env::args().collect();

    // CAN BE USED TO TEST THIS PROGRAM THROUGH COMMAND LINE INSTEAD OF ENVIRONMENT VARIABLES
    //let birthday_date: NaiveDate = get_birthday_from_env(Some(args[1].to_owned()));
    
    // the actual environment variable using call for function 
    let birthday_date: NaiveDate = get_birthday_from_env(None);

    let today: NaiveDate = Local::now().date_naive();
    
    if today.day() == birthday_date.day() && today.month() == birthday_date.month() {
        print!("Happy birthday!\n");
    }

    let day_difference: i32 = today.to_epoch_days() - birthday_date.to_epoch_days();
    if day_difference > 0 {
        print!("You are {} days old!\n", day_difference);
    }
    else if day_difference == 0 {
        print!("Looks like you're new here\n");
    }
    else {
        print!("Are you from the future?\n");
    }


    if day_difference % 1000 == 0 && day_difference != 0 {
        print!("That's a nice, round number!\n");
    }

}
