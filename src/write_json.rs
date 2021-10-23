use std::{fs::File, io::Write};
use std::collections::HashMap;
use serde_json::json;

// use std::io::{BufWriter, stdout, Write};
use crate::network::{self, VertexId};

pub fn get_output_val(network : &network::Network, vertex_id_to_name : HashMap<VertexId, String>) -> serde_json::Value{
    assert!(vertex_id_to_name.len() > 0);
    json!({
        "arrival_times": network.arrival_times
    })
}


pub fn write_json(network : &network::Network, vertex_id_to_name : HashMap<VertexId, String>, results_fname : &str){
    let output_val = get_output_val(network, vertex_id_to_name);
    let mut results_json = File::create(results_fname).unwrap();
    results_json.write_all(output_val.to_string().as_bytes()).expect("Failed to write results to json");
}