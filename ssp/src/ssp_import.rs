

use tempfile::TempDir;

use ssp_schema::ssp_1::ssd::SystemStructureDescription;
use fmi::fmi3::import::Fmi3Import;
use fmi::fmi2::import::Fmi2Import;
//
pub enum Resource {
    Fmi3(Fmi3Import),
    Fmi2(Fmi2Import),
    Ssp(SspImport),
    SspSsd(SystemStructureDescription),
}

#[allow(dead_code)]
pub struct SspImport {
    pub original_path: Option<String>,
    pub temp_path: TempDir,
    pub ssd: SystemStructureDescription,
    pub resources: Vec<Resource>,
}
