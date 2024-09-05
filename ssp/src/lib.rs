pub mod ssp_import;

use fmi::fmi2::import::Fmi2Import;
use fmi::traits::FmiImport;
use ssp_import::{Resource, SspImport};
use ssp_schema::ssp_1::ssd::{ComponentKind, SystemStructureDescription};
use ssp_schema::ResultBox;
use tempfile::TempDir;

use std::path::Path;
use std::process::Command;
use std::{
    fs::File,
    io::{BufReader, Read, Seek},
};

fn deserialize_ssd(temp_path: &Path) -> Result<SystemStructureDescription, String> {
    let ssd_path = temp_path.join("SystemStructure.ssd");

    if ssd_path.exists() {
        let ssd = SystemStructureDescription::from_path(&ssd_path.to_str().unwrap())
            .expect("Failed to parse sdd");

        return Ok(ssd);
    } else {
        Err(String::from("SSD file does not exist"))
    }
}

fn get_resources(ssd: &SystemStructureDescription, temp_path: &Path) -> Vec<Resource> {
    let mut resources = Vec::new();

    let elements = ssd.System.Elements.as_ref().expect("No elements found");
    let components = &elements.Components;

    for component in &components[..2] {
        let c_type = component
            .component_type
            .as_ref()
            .expect("Could not extract component type");

        let comp_path = temp_path.join(component.source.clone());
        let md = comp_path.metadata().expect("Metadata call failed");

        log::debug!("Resource found: {:#?}", comp_path);
        log::debug!("Component path exists {}", comp_path.exists());
        log::debug!("{:#?}", md.permissions());

        let resource = import_resource(&c_type, comp_path.to_str().unwrap());
        resources.push(resource);
    }
    return resources;
}

fn import_resource(c_type: &ComponentKind, path: &str) -> Resource {
    match c_type {
        ComponentKind::XFmuSharedlibrary(_) => {
            log::debug!("Extracting fmu: {path:#?}");

            let import = fmi::import::from_path::<Fmi2Import>(path).expect("Failed to parse fmu");
            log::debug!("Extracting successful into {:#?}", import.archive_path());

            Resource::Fmi2(import)
        }
        ComponentKind::XSspDefinition(_) => todo!(),
        ComponentKind::XSspPackage(_) => todo!(),
    }
}

impl SspImport {
    pub fn new<R: Read + Seek>(reader: R) -> ResultBox<Self> {
        log::info!("Importing new SSP");
        let mut archive = zip::ZipArchive::new(reader).expect("Failed to open SSP reader");
        let temp_dir = tempfile::Builder::new().prefix("ssp-").tempdir()?;
        let temp_path = temp_dir.path();

        log::info!("Extracting into {temp_dir:?}");
        archive.extract(&temp_dir)?;

        let ssd = deserialize_ssd(temp_path).expect("Failed to deserilize ssd");

        let resources = get_resources(&ssd, temp_path);

        let ssp = SspImport {
            original_path: None,
            temp_path: temp_dir,
            ssd: ssd,
            resources: resources,
        };

        return Ok(ssp);
    }

    pub fn from_path(path: &str) -> ResultBox<Self> {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        let mut ssp = SspImport::new(reader).expect("Failed to open SSP file");

        ssp.original_path = Some(path.to_string());

        return Ok(ssp);
    }
}
