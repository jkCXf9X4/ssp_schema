
#![allow(unused_imports)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};

use fmi::fmi2::import::Fmi2Import;
use fmi::traits::FmiImport;
use petgraph::data::Build;
use petgraph::dot::{Config, Dot};
use petgraph::Graph;


use ssp::ssp_import::SspImport;
use ssp_schema::ssp_1::ssd::SystemStructureDescription;


fn main() -> std::io::Result<()> {
    env_logger::init();
    log::info!("Main!");

    let ssp: SspImport = SspImport::from_path("ssp_test/reference_ssd/embrace.ssp").expect("Failed to parse ssd");

    log::info!("Storing internal representation");
    let mut output_file = File::create("ssp_test/reference_ssd/parsed_xml")?;
    write!(output_file, "{:#?}", ssp.ssd)?;

    log::info!("Creating graph");
    let mut graph = Graph::<&str, &str>::new();
    
    log::info!("Creating nodes");
    let components  = ssp.ssd.System.Elements.unwrap().Components;

    let mut comp_id = HashMap::new();
    let mut id_comp = HashMap::new();

    for component in components.iter()
    {
        let name = component.name.as_deref().expect("No name attribute found");
        // .unwrap();
        let id = graph.add_node(name);
        
        log::debug!("Node: {:#?}", name);
        comp_id.insert(name, id);
        id_comp.insert(id, name);

    }

    log::info!("Creating edges");

    let connections = ssp.ssd.System.Connections.unwrap().list;
    for connection in connections.iter()
    {
        let start_node = connection.startElement.as_deref().expect("No start element existing");
        let end_node = connection.endElement.as_deref().expect("No end element existing");
        
        log::debug!("Edge: {:#?} ->  {:#?}", start_node, end_node);
        // log::info!("{:#?}", end_node);

        graph.add_edge(comp_id[start_node], comp_id[end_node], "");

    }


    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let tarjan_scc = petgraph::algo::tarjan_scc(&graph);

    for i in tarjan_scc {
        println!("new scc");
        for j in i {

            println!("{:#?}", id_comp[&j]);
        }
    }

    
    
    
    return Ok(());
}
