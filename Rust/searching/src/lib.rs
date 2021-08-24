// pub mod disk_orbifolds_lib;

pub mod denominators_lib;
pub mod points_order_and_occurences_lib;

pub mod common_lib;

// use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
// use std::io::prelude::*;

use std::path::Path;
// use std::path::Path;



pub fn read_config<C: serde::de::DeserializeOwned>(lib: String) -> Result<C, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(Path::new(&("config/".to_string() + &lib + ".json")))?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instancse of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}
