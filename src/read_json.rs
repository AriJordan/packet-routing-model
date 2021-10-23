use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::collections::VecDeque;
use serde_json;

use crate::fraction::Fraction;
use crate::network::{Network, Vertex, Edge, Packet, VertexId, EdgeId};

pub fn map_vertex_name_id(network_val : &serde_json::Value) -> (HashMap<&str, VertexId>, HashMap<VertexId, String>){
    let mut v_name_to_id : HashMap<&str, VertexId> = HashMap::<&str, VertexId>::new();
    let mut v_id_to_name : HashMap<VertexId, String> = HashMap::<VertexId, String>::new();
    
    for edge_val in network_val["edges"].as_array().unwrap(){
        let v_from_name = edge_val["v_from"].as_str().unwrap();
        let v_to_name = edge_val["v_to"].as_str().unwrap();
        for v_name in [v_from_name, v_to_name]{
            if !v_name_to_id.contains_key(v_name){
                let new_id = v_name_to_id.len();
                v_name_to_id.insert(v_name, new_id);
                v_id_to_name.insert(new_id, v_name.to_string());
            }
        }
    }
    assert_eq!(v_name_to_id.len(), v_id_to_name.len());
    (v_name_to_id, v_id_to_name)
}

pub fn get_network(network_val : &serde_json::Value, vertex_name_to_id : &HashMap<&str, VertexId>) -> (Vec::<Edge>, Vec::<Vertex>, HashMap::<(VertexId, VertexId), EdgeId>){
    let n_vertices : usize = vertex_name_to_id.len();
    let mut edges : Vec::<Edge> = Vec::<Edge>::new();
    let mut vertices : Vec<Vertex> = vec![Vertex{incoming_edges : Vec::<VertexId>::new(), outgoing_edges : Vec::<VertexId>::new()}; n_vertices];
    let mut edge_to_id = std::collections::HashMap::<(VertexId, VertexId), EdgeId>::new();
    for (edge_id, edge_val) in network_val["edges"].as_array().unwrap().iter().enumerate(){
        let v_from = vertex_name_to_id[edge_val["v_from"].as_str().unwrap()];
        assert!(v_from < n_vertices, "vertex indices should be in [0, n_vertices)");
        let v_to = vertex_name_to_id[edge_val["v_to"].as_str().unwrap()];
        assert!(v_to < n_vertices, "vertex indices should be in [0, n_vertices)");
        let length = edge_val["transit_time"].as_f64().unwrap() as usize; // TODO: handle fractional
        assert!(length > 0, "edge lengths should be positive");
        let capacity = edge_val["capacity"].as_f64().unwrap() as i64; // TODO: handle fractional
        assert!(capacity > 0, "edge capacities should be positive");
        edges.push(
            Edge{
                id : edge_id,
                v_from : v_from,
                v_to : v_to,
                length : length,
                average_capacity : Fraction{numerator : capacity, denominator : 1},
                current_capacity : Fraction{numerator : capacity, denominator : 1},        
            }
        );
        vertices[v_from].outgoing_edges.push(edge_id);
        vertices[v_to].incoming_edges.push(edge_id);
        edge_to_id.insert((v_from, v_to), edge_id);
    }
    (edges, vertices, edge_to_id)
}

pub fn get_packets(packets_val : &serde_json::Value, v_name_to_id : &HashMap<&str, VertexId>, edge_to_id : &HashMap::<(VertexId, VertexId), EdgeId>) -> Vec::<Packet>{
    let mut packets = Vec::<Packet>::new();
    for (packet_id, packet_val) in packets_val["packets"].as_array().unwrap().iter().enumerate() {
        let release_time = packet_val["release_time"].as_u64().unwrap() as usize;
        let path_length = packet_val["path"].as_array().unwrap().len();
        let vertex_path : Vec<VertexId> = (0..path_length).map(|i| v_name_to_id[packet_val["path"].as_array().unwrap()[i].as_str().unwrap()] as VertexId).collect();
        assert!(vertex_path.len() >= 2, "paths should have length at least 2");
        packets.push(
            Packet{
                id : packet_id,
                release_time : release_time,
                path : vertex_path_to_edge_path(vertex_path, &edge_to_id),
                entrance_time : None,
                path_position : None,
            }
        );
    }
    packets
}

fn vertex_path_to_edge_path(vertex_path : Vec<VertexId>, edge_to_id : &std::collections::HashMap::<(VertexId, VertexId), EdgeId>) -> Vec<EdgeId>{
    assert!(vertex_path.len() > 0);
    let mut edge_path = Vec::<EdgeId>::new();
    for id in 0..vertex_path.len()-1{
        edge_path.push(edge_to_id[&(vertex_path[id], vertex_path[id + 1])]);
    }
    assert_eq!(edge_path.len(), vertex_path.len() - 1);
    edge_path
}

pub fn read_json(network_fname : &str, packets_fname : &str) -> (Network, HashMap<VertexId, String>){

    // json files for network and packets
    let mut network_json = File::open(network_fname).unwrap();
    let mut packets_json = File::open(packets_fname).unwrap();

    // Read into strings
    let mut network_string = String::new();
    network_json.read_to_string(&mut network_string).unwrap();
    let mut packets_string = String::new();
    packets_json.read_to_string(&mut packets_string).unwrap();

    // Deserialize
    let network_val : serde_json::Value = serde_json::from_str(&network_string).unwrap();
    #[cfg(debug_assertions)]
    println!("network_de[\"edges\"][0] = {:?}", network_val["edges"][0]);
    let packets_val : serde_json::Value = serde_json::from_str(&packets_string).unwrap();
    #[cfg(debug_assertions)]
    println!("packets_de[\"packets\"][0] = {:?}", packets_val["packets"][0]);

    // Map vertex names to ints and vice versa
    let (vertex_name_to_id, vertex_id_to_name) = map_vertex_name_id(&network_val);

    // Convert to own network class
    let (edges, vertices, edge_to_id) = get_network(&network_val, &vertex_name_to_id);
    let packets = get_packets(&packets_val, &vertex_name_to_id, &edge_to_id);
    let network = Network{
        edge_queues : vec![VecDeque::new(); vertices.len()],
        leaving_queues: vec![VecDeque::new(); vertices.len()],
        vertices : vertices,
        edges : edges,
        arrival_times : vec![None; packets.len()],  
        packets : packets,
        time : 0,
        packets_arrived : 0,
    };
    (network, vertex_id_to_name)
}

#[test]
fn test_read_json(){
    let folder = "src/instances/instance_zimmer/";
    let (network, vertex_id_to_name) = read_json(&(folder.to_owned() + "network.json"), &(folder.to_owned() + "packets.json"));
    assert_eq!(network.vertices.len(), vertex_id_to_name.len()); // One name per vertex
    assert_eq!(network.vertices.len(), 6);
    assert_eq!(network.edges.len(), 5);
    //let mut network = input::input("src/instances/instance_l.txt");
}