
// extern crate yaserde;

use std::fs::File;
use std::io::{BufReader, Write};

// use std::io::prelude::*;
// use xml_schema_derive::XmlSchema;
// use yaserde::{YaDeserialize, YaSerialize};
// use std::collections::BTreeMap;


// use yaserde::YaDeserialize;

mod ssd;
mod ssc;
use ssd::*;


fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let f = File::open("ssd/SystemStructure.ssd").unwrap();
    let reader = BufReader::new(f);
    
    let ssd: SystemStructureDescription = yaserde::de::from_reader(reader).unwrap();
    let s = format!("{:#?}", ssd);

    let mut output_file = File::create("parsed_xml")?;

    write!(output_file, "{}", s)?;


    return Ok(());
}
