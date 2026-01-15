//Function meant to split string type date into tuple of three u16 size numbers in (year, month, day) format
fn split_date(date: String) -> (u16, u16, u16) {
    let date_len: usize = date.len();
    let event_day_str = &date[date_len-2..date_len];
    let event_month_str = &date[date_len-4..date_len-2];
    let event_year_str = &date[0..date_len-4];
    return (event_year_str.parse().unwrap(), event_month_str.parse().unwrap(), event_day_str.parse().unwrap());
}

// Give day from which we want to get the events for. Full date given to get both month and day
fn print_events_for_date(date_to_print: (u16, u16, u16), events: &[(i32, &str); 12]) {
    let mut title: bool = false;
    for event in events {
        let event_date: (u16, u16, u16) = split_date(event.0.to_string());
        if date_to_print.1 == event_date.1 && date_to_print.2 == event_date.2 {
            // Otsikko
            if !title {
                println!("Events for date {}.{}", date_to_print.2, date_to_print.1);
                title = true;
            }
            println!("{} {}", event_date.0, event.1);
        }
    }
}
/* 
Give tuple in format <month, start_day, end_day> 
Does not take into account month changes, but neither does anything else in this code
*/
 fn print_full_weeks_events(day_range: (u32, u32), events: &[(i32, &str); 12]) {
    for day in day_range.0..day_range.1 {
        let day_tuple: (u16, u16, u16) = split_date(day.to_string());
        let mut title: bool= false;
        for event in events {
            let event_date: (u16, u16, u16) = split_date(event.0.to_string());
            if day_tuple.1 == event_date.1 && day_tuple.2 == event_date.2 {
                // Otsikko
                if !title {
                    println!("{}.{} events:", day_tuple.2, day_tuple.1);
                    title = true;
                }

                println!("{} {}", event_date.0, event.1);
            }
        }
        println!();
    }
}

fn main() {
    let current_date_str = String::from("20260118");
    let current_date: (u16, u16, u16) = split_date(current_date_str);
    let events:[(i32, &str); 12] = [
        (2001_01_15, "Wikipedia julkaistu, Jimmy Wales ja Larry Sangeri"),
        (1920_01_16, "Ensimmäinen kansainliiton kokous pariisissa"),
        (1991_01_17, "Operaatio Desert Storm, yhdysvallat sekoilee"),
        (1919_01_18, "Pariisin rauhankonferenssi ww1 jälkeen"),
        (1966_01_19, "Intia sai ensimmäisen naispuolisen pääministerin"),
        (2009_01_20, "Obama presidentiksi"),
        (1793_01_21, "Ranskan kuningas Ludvig XVI teloitettiin"),
        (1506_01_22, "Ensimmäinen sveitsiläiskaartin yksikkö saapuu vatikaaniin"),
        (1240_02_20, "testi listan ulkopuolelta"), // Alkaa testit 3 kpl
        (1950_01_23, "testi listan ulkopuolelta 2"),
        (1950_01_14, "testi listan ulkopuolelta 3"),
        (1943_01_18, "Leningradin piiritys murrettu"),
    ];

    print_events_for_date(current_date, &events);
    
    println!("\nNext is full week print test.\n");
    
    print_full_weeks_events((20260115, 20260122), &events);
}