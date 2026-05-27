use crate::EventProvider;
use crate::events::{Category, Event};
use chrono::NaiveDate;
use csv::ReaderBuilder;
use csv::WriterBuilder;
use std::error::Error;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

pub struct CSVFileProvider {
    name: String,
    path: PathBuf,
}

impl CSVFileProvider {
    pub fn new(name: &str, path: &Path) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf(),
        }
    }
}

impl EventProvider for CSVFileProvider {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn get_events(&self, events: &mut Vec<Event>) {
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_path(self.path.clone())
            .expect("existing CSV file");
        for result in reader.records() {
            let record = result.unwrap();
            let date_string = record[0].to_string();
            let description = record[1].to_string();
            let category_string = record[2].to_string();
            match NaiveDate::parse_from_str(&date_string, "%F") {
                Ok(date) => {
                    let category = Category::from_str(&category_string);
                    let event = Event::new_singular(date, description.clone(), category);
                    events.push(event);
                }
                Err(_) => {
                    eprintln!("Invalid date '{}'", date_string);
                }
            }
        }
    }

    fn add_event(
        &self,
        date: NaiveDate,
        description: &str,
        category: &Category,
    ) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .from_writer(file);

        writer.write_record(&[
            date.format("%F").to_string(),
            description.to_string(),
            category.to_string(),
        ])?;
        writer.flush()?;

        Ok(())
    }
}
