use std::collections::VecDeque;
use crate::network::{Network, Vertex, Edge, Packet, VertexId, EdgeId, Time};
use crate::scanner::Scanner;
use crate::fraction::Fraction;

fn vertex_path_to_edge_path(vertex_path : Vec<VertexId>, edge_to_id : &std::collections::HashMap::<(VertexId, VertexId), EdgeId>) -> Vec<EdgeId>{
    assert!(vertex_path.len() > 0);
    let mut edge_path = Vec::<EdgeId>::new();
    for id in 0..vertex_path.len()-1{
        edge_path.push(edge_to_id[&(vertex_path[id], vertex_path[id + 1])]);
    }
    assert_eq!(edge_path.len(), vertex_path.len() - 1);
    edge_path
}

// Loads network from filename
/// First line contains number of vertices and edges
/// The next #edges number of lines contain the edges
/// in the format (v_from, v_to, length, capacity)
/// The next line contains the numper of packets
/// Each packet is described in three lines:
/// line 1: release time of packet
/// line 2: number of vertices on packet's path
/// line 3: path of the packet as the vertices on it
pub fn input(filename : &str) -> Network{
    //env::set_var("RUST_BACKTRACE", "full");
    let input_string = std::fs::read_to_string(filename).expect("Error while reading file");
    let buffer : Vec<String> = input_string.split_whitespace().rev().map(String::from).collect();
    let mut scan = Scanner{buffer : buffer};
    let n_vertices : usize = scan.next::<usize>();
    let n_edges : usize = scan.next::<usize>();
    let mut edges : Vec::<Edge> = Vec::<Edge>::new();
    let mut vertices : Vec<Vertex> = vec![Vertex{incoming_edges : Vec::<VertexId>::new(), outgoing_edges : Vec::<VertexId>::new()}; n_vertices];
    let mut edge_to_id = std::collections::HashMap::<(VertexId, VertexId), EdgeId>::new();
    for edge_id in 0..n_edges{
        let v_from = scan.next::<VertexId>();
        assert!(v_from < n_vertices, "vertex indices should be in [0, n_vertices)");
        let v_to = scan.next::<VertexId>();
        assert!(v_to < n_vertices, "vertex indices should be in [0, n_vertices)");
        let length = scan.next::<usize>();
        assert!(length > 0, "edge lengths should be positive");
        let capacity = scan.next::<i64>();
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

    let n_packets : usize = scan.next::<usize>();
    let mut packets = Vec::<Packet>::new();
    for packet_id in 0..n_packets{
        let release_time = scan.next::<Time>();
        let path_length = scan.next::<usize>();
        let vertex_path : Vec<VertexId> = (0..path_length).map(|_| scan.next::<VertexId>()).collect();
        assert!(vertex_path.len() >= 2, "paths should have length at least 2");
        packets.push(
            Packet{
                id : packet_id,
                commodity_id : commodity_id,
                release_time : release_time,
                path : vertex_path_to_edge_path(vertex_path, &edge_to_id),
                entrance_time : None,
                path_position : None,
            }
        );
    }
    let network = Network{
        vertices : vertices,
        edges : edges,    
        packets : packets,
        edge_queues : vec![VecDeque::new(); n_edges],
        leaving_queues: vec![VecDeque::new(); n_edges],
        time : 0,
        packets_arrived : 0,
        arrival_times : vec![None; n_packets],
    };
    network
}