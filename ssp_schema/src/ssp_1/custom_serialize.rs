
use std::io::Read;

use yaserde::YaDeserialize;

use xml::reader::XmlEvent;

use super::ssd::ComponentKind;

// ComponentKind

#[allow(dead_code)]
impl ComponentKind {
    pub fn from_string(kind: &String) -> Result<Self, &'static str> {
        match kind.as_str() {
            "application/x-fmu-sharedlibrary" => Ok(Self::XFmuSharedlibrary(kind.to_string())),
            "application/x-ssp-definition" => Ok(Self::XSspDefinition(kind.to_string())),
            "application/x-ssp-package" => Ok(Self::XSspPackage(kind.to_string())),
            _ => Err("Invalid type for ComponentKind"),
        }
    }

    pub fn to(self) -> String{
        match self {
            Self::XFmuSharedlibrary(s) => s,
            Self::XSspDefinition(s) => s,
            Self::XSspPackage(s) => s,
        }
    }
}

impl YaDeserialize for ComponentKind {
    fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
      
      let _ = reader.next_event();
      let event = reader.next_event()?;
      
      if let XmlEvent::Characters(s) = &event {
        let t = Self::from_string(s).expect(format!("Failed to convert type form {}", s).as_str());

        // println!("{:#?}", t);
        return Ok(t);
      }
      Err(String::from("Failed to parse ComponentKind"))
    }
  }