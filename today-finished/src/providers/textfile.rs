use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::Category;
use crate::Event;
use crate::providers::EventProvider;

enum ReadingState {
    Date,
    Description,
    Category,
    Separator,
}

pub struct TextFileProvider {
    name: String,
    path: PathBuf,
}

impl TextFileProvider {
    pub fn new(name: &str, path: &Path) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf(),
        }
    }
}

impl EventProvider for TextFileProvider {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn get_events(&self, events: &mut Vec<Event>) {
        let f = File::open(self.path.clone()).expect("existing text file");
        let reader = BufReader::new(f);
        let mut state = ReadingState::Date;
        let mut date_string = String::new();
        let mut description = String::new();
        let mut category_string = String::new();
        for line_result in reader.lines() {
            let line = line_result.expect("read line");
            match state {
                ReadingState::Date => {
                    date_string = line;
                    state = ReadingState::Description;
                }
                ReadingState::Description => {
                    description = line;
                    state = ReadingState::Category;
                }
                ReadingState::Category => {
                    category_string = line;
                    state = ReadingState::Separator;
                }
                ReadingState::Separator => {
                    match chrono::NaiveDate::parse_from_str(&date_string, "%F") {
                        Ok(date) => {
                            let category = Category::from_str(&category_string);
                            let event = Event::new_singular(date, description.clone(), category);
                            events.push(event);
                        }
                        Err(_) => {
                            eprintln!("Invalid date '{}'", date_string);
                        }
                    }
                    state = ReadingState::Date;
                }
            }
        }
    }

    fn add_event(
        &self,
        date: chrono::NaiveDate,
        description: &str,
        category: &Category,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        writeln!(file, "{}", date.format("%F"))?;
        writeln!(file, "{}", description)?;
        writeln!(file, "{}", category)?;
        writeln!(file)?;

        Ok(())
    }
}
