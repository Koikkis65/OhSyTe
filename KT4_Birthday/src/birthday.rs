use chrono::prelude::*;
use std::env;


/*
    Might look funky, but it was made so I could test different dates and how the program reacts
    Mostly because vscode and environment variables do not work together well.
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