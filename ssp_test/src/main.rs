
#![allow(unused_imports)]

use std::fs::File;
use std::io::{BufReader, Write};

// use petgraph::Graph;


// use ssp::SSP;
use ssp_schema::ssp::ssd::SystemStructureDescription;


fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let ssp = SystemStructureDescription::new_from_file("ssp_test/reference_ssd/SystemStructure.ssd").expect("Failed to parse ssd");



    let mut output_file = File::create("ssp_test/reference_ssd/parsed_xml")?;
    write!(output_file, "{:#?}", ssp)?;

    
    // let mut deps = Graph::<&str, &str>::new();

    // let components  = ssd.System.Elements.unwrap().Components;

    // for component in components.iter()
    // {
    //     println!("{:#?}", component);
    // }

    // return Ok(());
    
    // let conections = ssd.System.Connections.unwrap().list;
    // for connection in conections.iter()
    // {
    //     println!("{:#?}", connection);

    // }



    return Ok(());
}
