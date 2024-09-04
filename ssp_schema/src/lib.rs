pub mod ssp_1 ;

use std::{
    error, fs::File, io::BufReader
};

use log;

pub type ResultBox<T> = std::result::Result<T, Box<dyn error::Error>>;

use ssp_1::ssd::SystemStructureDescription;


impl SystemStructureDescription {
    pub fn from_path(path: &str) -> ResultBox<Self> {
        log::info!("Parsing new SSD from file");
        let f = File::open(path).expect("Failed to open SSD");
        let reader = BufReader::new(f);

        let ssd: SystemStructureDescription = yaserde::de::from_reader(reader).expect("Failed to parse SSD");
        
        return Ok(ssd);
    }
}

