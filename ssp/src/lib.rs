

use std::{io::Read};

pub use ssp_schema;

use ssp_schema::ssp::ssd::SystemStructureDescription;

pub struct SspImport
{
    temp_path : String,
    ssd : SystemStructureDescription,

}


pub fn new<R: Read + Seek>(reader: R) -> Result<Imp, Error> {
    let mut archive = zip::ZipArchive::new(reader)?;
    let temp_dir = tempfile::Builder::new().prefix("-rs").tempdir()?;
    log::debug!("Extracting into {temp_dir:?}");
    archive.extract(&temp_dir)?;

    // Open and read the modelDescription XML into a string
    let f = File::open("ssd/SystemStructure.ssd").unwrap();
    let reader = BufReader::new(f);
    
    let ssd: SystemStructureDescription = yaserde::de::from_reader(reader).unwrap();

    Imp::new(temp_dir, &descr_xml)
}