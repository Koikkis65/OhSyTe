#![allow(dead_code, unused_variables, unused_imports, unused_mut)]
use chrono::{NaiveDate, Datelike, Local};
use std::env;


/*
    Might look funky, but it was made so I could test different dates and how the program reacts
    Mostly because vscode and environment variables do not work together well. Meaning need to restart vscode to test changing env variables.
    below code allows you to test the program by doing something like .\main.exe 18.05.2001
*/
pub fn get_birthday_from_env(env_var: Option<String>) -> NaiveDate {
    let env_var_bday = match env_var {
        None => match env::var("BIRTHDATE") {
            Ok(val) => val,
            Err(error) => panic!("BIRTHDATE environment variable not set"),
        },
        Some(val) => val,
    };

    // Read birthday and handle wrong input
    let birthday_date = match NaiveDate::parse_from_str(&env_var_bday, "%Y-%m-%d") {
        Ok(val  ) => val,
        Err(error) => panic!("Parse error on birthday env variable: {error:?}"),
    };

    return birthday_date;
}

pub fn is_birthday_today(birthday: NaiveDate) -> bool {
    let today: NaiveDate = Local::now().date_naive();
    
    if today.day() == birthday.day() && today.month() == birthday.month() {
        print!("Happy birthday!\n");
    }
    else {
        return false;
    }

    let day_difference: i32 = today.to_epoch_days() - birthday.to_epoch_days();
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
    return true;
}