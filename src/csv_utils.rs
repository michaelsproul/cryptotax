use csv::Trim;
use serde::de::DeserializeOwned;
use std::error::Error;
use std::fs::File;

pub fn get_rows<R>(filename: &str) -> Result<Vec<R>, Box<Error>>
where
    R: DeserializeOwned,
{
    let file = File::open(filename)?;
    let mut rdr = csv::ReaderBuilder::new()
        .trim(Trim::Headers)
        .from_reader(file);

    let mut rows = vec![];
    for result in rdr.deserialize() {
        rows.push(result?);
    }

    Ok(rows)
}
