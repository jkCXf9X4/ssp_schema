use ssp_schema::ssp::ssd::SystemStructureDescription;
use ssp_schema::ResultBox;
use tempfile::TempDir;

use std::{
    fs::File,
    io::{BufReader, Read, Seek},
};

#[allow(dead_code)]
pub struct SSP {
    original_path: Option<String>,
    temp_path: TempDir,
    ssd: SystemStructureDescription,
}

impl SSP {
    pub fn new<R: Read + Seek>(reader: R) -> ResultBox<Self> {
        log::trace!("Importing new SSP");
        let mut archive = zip::ZipArchive::new(reader).expect("Failed to open SSP reader");
        let temp_dir = tempfile::Builder::new().prefix("-rs").tempdir()?;
        log::trace!("Extracting into {temp_dir:?}");
        archive.extract(&temp_dir)?;

        let ssd = SystemStructureDescription::new_from_file("ssd/SystemStructure.ssd")
            .expect("Failed to open SSD");

        // let components = ssd.System.Elements.expect("Cant access elemets").Components;

        // for component in components
        // {

        // }

        let ssp = SSP {
            original_path: None,
            temp_path: temp_dir,
            ssd: ssd,
        };
        return Ok(ssp);
    }

    pub fn new_from_file(path: &str) -> ResultBox<Self> {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        let mut ssp = SSP::new(reader).expect("Failed to open SSP file");
        ssp.original_path = Some(path.to_string());
        return Ok(ssp);
    }
}
