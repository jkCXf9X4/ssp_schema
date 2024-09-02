
// extern crate yaserde;

use std::fs::File;
use std::io::{BufReader, Write};

// use std::io::prelude::*;
// use xml_schema_derive::XmlSchema;
// use yaserde::{YaDeserialize, YaSerialize};
// use std::collections::BTreeMap;


// use yaserde::YaDeserialize;

use petgraph::Graph;

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

    
    let mut deps = Graph::<&str, &str>::new();

    let components  = ssd.System.Elements.unwrap().Components;

    for component in components.iter()
    {
        println!("{:#?}", component);
    }

    return Ok(());
    
    let conections = ssd.System.Connections.unwrap().list;
    for connection in conections.iter()
    {
        println!("{:#?}", connection);

    }



    return Ok(());
}
