use crate::aircraft::AircraftState;
use std::error::Error;
use std::fs::File;

pub fn export_to_csv(filename: &str, data: &[AircraftState]) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?;
    let mut wtr = csv::Writer::from_writer(file);

    for record in data {
        wtr.serialize(record)?;
    }

    wtr.flush()?;
    Ok(())
}