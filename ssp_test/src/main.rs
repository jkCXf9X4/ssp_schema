
#![allow(unused_imports)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};

use petgraph::data::Build;
use petgraph::dot::{Config, Dot};
use petgraph::Graph;


// use ssp::SSP;
use ssp_schema::ssp::ssd::SystemStructureDescription;


fn main() -> std::io::Result<()> {
    log::trace!("Main!");

    log::trace!("Parsing ssd");
    let ssd = SystemStructureDescription::new_from_file("ssp_test/reference_ssd/SystemStructure.ssd").expect("Failed to parse ssd");

    log::trace!("Storing internal repressentation");
    let mut output_file = File::create("ssp_test/reference_ssd/parsed_xml")?;
    write!(output_file, "{:#?}", ssd)?;

    log::trace!("Creating graph");
    let mut graph = Graph::<&str, &str>::new();
    
    log::trace!("Creating nodes");
    let components  = ssd.System.Elements.unwrap().Components;

    let mut comp_id = HashMap::new();

    for component in components.iter()
    {
        let name = component.name.as_deref().expect("No name attribute found");
        // .unwrap();
        let id = graph.add_node(name);

        comp_id.insert(name, id);

        println!("{:#?}", name);
    }

    log::trace!("Creating edges");

    let conections = ssd.System.Connections.unwrap().list;
    for connection in conections.iter()
    {
        let start_node = connection.startElement.as_deref().expect("No start elelent existing");
        let end_node = connection.endElement.as_deref().expect("No end elelent existing");
        
        println!("{:#?}", start_node);
        println!("{:#?}", end_node);

        graph.add_edge(comp_id[start_node], comp_id[end_node], "");

    }
    
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    
    
    return Ok(());
}
