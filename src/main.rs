#[allow(unused_imports)]
use std::cmp::{min,max};
use std::io::stdin;
// use std::io::{BufWriter, stdout, Write};
use std::collections::VecDeque;
use std::default::Default;

const USIZE_DEFAULT: usize = std::usize::MAX;

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

type Time = usize;
type VertexId = usize;
type EdgeId = usize;
type PacketId = usize;

#[derive(Clone)]
struct Edge{
    id : EdgeId,
    v_from : VertexId,
    v_to : VertexId,
    length : usize,
    capacity : f64,
}



#[derive(Clone)]
struct Packet{
    id : PacketId,
    release_time : Time,
    path : Vec<EdgeId>, // edges on path of packet
    entrance_time : Option<Time>,
    path_position : Option<usize>, // index in path : Vec<usize>
}

#[derive(Clone)]
struct Vertex{
    incoming_edges : Vec<EdgeId>,
    outgoing_edges : Vec<EdgeId>,
}

struct Network{
    vertices : Vec<Vertex>,
    edges : Vec<Edge>,
    packets : Vec<Packet>,
    queues : Vec<VecDeque<PacketId>>, // i-th queue corresponds to i-th edge
    time : Time,
    packets_arrived : usize,
    arrival_times : Vec<Time>,
}

// TODO: separaze Network into structs Network + State

impl Network{

    fn run_simulation(&mut self){
        while self.packets_arrived < self.packets.len(){
            self.timestep();
        }
    }

    fn timestep(&mut self){
        // TODO: split into two parts: movement and queueing
        for (packet_id, packet) in self.packets.iter_mut().enumerate(){
            if packet.release_time > self.time{
                // Packet not ready yet
                continue;
            }
            else{
                match packet.path_position{
                    None =>{                        
                        if packet.release_time == self.time{
                            // Enter first edge on path, TODO: order compared to other packets entering edge
                            let path_position = 0;  
                            packet.path_position = Some(path_position);
                            let edge_id = packet.path[path_position];
                            packet.entrance_time = Some(self.time);                            
                            self.queues[edge_id].push_back(packet_id);                    
                        }
                    }
                    Some(path_position) =>{
                        let edge_id = packet.path[path_position];
                        match packet.entrance_time{
                            None => println!("Error: packet.entrance_time is None"),
                            Some(entrance_time) =>{
                                if self.time == entrance_time +  self.edges[edge_id].length{
                                    if path_position == packet.path.len() - 1{
                                        // packet at end of path
                                        packet.path_position = None;
                                        self.packets_arrived += 1;
                                        self.arrival_times[packet_id] = self.time;
                                    }
                                    else{
                                        // packet changes into next edge                                                                      
                                        let popped_packet_id = self.queues[edge_id].pop_front().expect("pop_front() on empty queue called");                                      
                                        self.queues[packet.path[path_position + 1]].push_back(popped_packet_id); // TODO: correct order
                                        packet.path_position = Some(path_position + 1); // TODO : check whether path_position is changed 
                                    }
                                }                           
                            }
                        }  
                    }
                }
            }
        }
        if self.packets_arrived < self.packets.len(){
            self.time += 1;
        }
    }
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

fn input(filename : &str) -> Network{
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
        let capacity = scan.next::<f64>();
        assert!(capacity > 0.0, "edge capacities should be positive");
        edges.push(
            Edge{
                id : edge_id,
                v_from : v_from,
                v_to : v_to,
                length : length,
                capacity : capacity,           
            }
        );
        vertices[v_from].outgoing_edges.push(v_to);
        vertices[v_to].incoming_edges.push(v_from);
        edge_to_id.insert((v_from, v_to), edge_id);
    }

    let n_packets : usize = scan.next::<usize>();
    println!("n_packets: {}", n_packets);
    let mut packets = Vec::<Packet>::new();
    for packet_id in 0..n_packets{
        let release_time = scan.next::<Time>();
        let path_length = scan.next::<usize>();
        let vertex_path : Vec<VertexId> = (0..path_length).map(|_| scan.next::<VertexId>()).collect();
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
    let network = Network{
        vertices : vertices,
        edges : edges,    
        packets : packets,
        queues : vec![VecDeque::new(); n_edges],
        time : 0,
        packets_arrived : 0,
        arrival_times : vec![USIZE_DEFAULT; n_packets],
    };
    network
}

fn output_arrivals(network : &Network){
    //let out = &mut BufWriter::new(stdout());
    println!("Last arrival time:\n{}\n\nIndividual arrival times:", network.arrival_times.iter().max().expect("max failed somehow"));
    for arrival in &network.arrival_times{
        print!("{} ", arrival);
    }
}

fn main() -> std::io::Result<()>{
    let mut network = input("src/instances/instance_l.txt");
    network.run_simulation();
    output_arrivals(&network);
    Ok(())
}

#[test]
fn test_testing(){
    assert!(true);
    assert_eq!(2 + 2, 4);
    assert_ne!(2 + 2, 5);
}

#[test]
fn test_empty(){
    let mut network = input("src/instances/instance_empty.txt");
    network.run_simulation();
    assert_eq!(network.vertices.len(), 0);
    assert_eq!(network.edges.len(), 0);
    assert_eq!(network.packets.len(), 0);
    assert_eq!(network.packets_arrived, 0);
    assert_eq!(network.arrival_times.len(), 0);
    assert_eq!(network.queues.len(), network.edges.len());
    assert_eq!(network.time, 0);
}

#[test]
fn test_i(){
    let mut network = input("src/instances/instance_i.txt");
    network.run_simulation();
    assert_eq!(network.vertices.len(), 2);
    assert_eq!(network.edges.len(), 1);
    assert_eq!(network.packets.len(), 1);
    assert_eq!(network.packets_arrived, 1);
    assert_eq!(network.arrival_times.len(), 1);
    assert_eq!(network.arrival_times[0], 1);
    assert_eq!(network.queues.len(), 1);
    assert_eq!(network.time, 1);
}