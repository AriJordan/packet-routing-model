use std::{fs::File, io::Write};
use std::collections::HashMap;
use serde_json::json;
use serde_json::to_string_pretty;

// use std::io::{BufWriter, stdout, Write};
use crate::network::{self, CommodityId, VertexId, Time};

pub fn get_output_val(network : &network::Network, vertex_id_to_name : HashMap<VertexId, String>) -> serde_json::Value{
    assert!(vertex_id_to_name.len() == network.vertices.len());
    json!({
        "commodity_ids": (0..network.packets.len()).map(|i| network.packets[i].commodity_id).collect::<Vec<CommodityId>>(),
        "arrival_times": network.arrival_times,
        "travel_times": (0..network.packets.len()).map(|i| network.arrival_times[i].unwrap() - network.packets[i].release_time).collect::<Vec<Time>>(),
    })
}


pub fn write_json(network : &network::Network, vertex_id_to_name : HashMap<VertexId, String>, results_fname : &str){
    let output_val = get_output_val(network, vertex_id_to_name);
    let mut results_json = File::create(results_fname).unwrap();
    results_json.write_all(to_string_pretty(&output_val).unwrap().as_bytes()).expect("Failed to write results to json");
}