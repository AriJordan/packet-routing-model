use std::fs::File;
use std::io::Read;
use serde_json;

pub fn read_json(network_fname : &str, packets_fname : &str){
    // json files for network and packets
    let mut network_json = File::open(network_fname).unwrap();
    let mut packets_json = File::open(packets_fname).unwrap();

    // Read into strings
    let mut network_string = String::new();
    network_json.read_to_string(&mut network_string).unwrap();
    let mut packets_string = String::new();
    packets_json.read_to_string(&mut packets_string).unwrap();

    // Deserialize
    let network_de : serde_json::Value = serde_json::from_str(&network_string).unwrap();
    let packets_de : serde_json::Value = serde_json::from_str(&packets_string).unwrap();
    println!("network_de[\"edges\"][0] = {:?}", network_de["edges"][0]);
    println!("packets_de[\"packets\"][0] = {:?}", packets_de["packets"][0]);
}