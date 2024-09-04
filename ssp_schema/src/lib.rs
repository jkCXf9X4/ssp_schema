pub mod ssp;

use std::{
    error, fs::File, io::BufReader
};

use log;

pub type ResultBox<T> = std::result::Result<T, Box<dyn error::Error>>;

use ssp::ssd::SystemStructureDescription;


impl SystemStructureDescription {
    pub fn new_from_file(path: &str) -> ResultBox<Self> {
        log::trace!("Parsing new SSD from file");
        let f = File::open(path).expect("Failed to open SSD");
        let reader = BufReader::new(f);

        let ssd: SystemStructureDescription = yaserde::de::from_reader(reader).expect("Failed to parse SSD");
        
        return Ok(ssd);
    }
}

