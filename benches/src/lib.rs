use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use csv::StringRecord;
use envconfig::Envconfig;
use examples::models::ExpectedResult;
use serde::de::DeserializeOwned;

pub type ExpectedResults = HashMap<String, HashMap<u32, HashMap<u32, ExpectedResult>>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Phase {
    Initial,
    Updates,
}

pub fn read_csv_update<T: TryFrom<StringRecord, Error = csv::Error>, P: AsRef<Path>>(
    path: P,
) -> Result<Vec<T>, csv::Error> {
    let mut rdr = csv_reader(path, true, '|')?;
    let mut records = Vec::new();
    for result in rdr.records() {
        let record: StringRecord = result?;
        records.push(record.try_into()?);
    }
    Ok(records)
}

pub fn read_csv_file<T: DeserializeOwned, P: AsRef<Path>>(
    path: P,
    delimiter: char,
) -> Result<Vec<T>, csv::Error> {
    let mut rdr = csv_reader(path, false, delimiter)?;
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: T = result?;
        records.push(record);
    }
    Ok(records)
}

fn csv_reader<P: AsRef<Path>>(
    path: P,
    flexible: bool,
    delimiter: char,
) -> Result<csv::Reader<std::fs::File>, csv::Error> {
    csv::ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .flexible(flexible)
        .has_headers(false)
        .from_path(path)
}

#[derive(Debug, Envconfig)]
pub struct SocialNetworkConfig {
    #[envconfig(from = "BENCH_DATA_DIR")]
    pub csv_dir: String,
    #[envconfig(from = "RESULTS_CSV_DIR", default = "data")]
    results_csv_dir: String,
}

impl SocialNetworkConfig {
    pub fn expected_results(&self) -> Result<ExpectedResults, csv::Error> {
        let results = read_csv_file::<ExpectedResult, _>(
            PathBuf::from(&self.results_csv_dir).join("social_network_results.csv"),
            ';',
        )?;
        Ok(results.into_iter().fold(HashMap::new(), |mut map, result| {
            let view_entry = map.entry(result.view.clone()).or_default();
            let changeset_entry = view_entry.entry(result.changeset).or_default();
            changeset_entry.insert(result.iteration, result);
            map
        }))
    }
}
