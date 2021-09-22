#[allow(unused_imports)]
use std::cmp::{min,max};
use std::io::{BufWriter, stdin, stdout, Write};
use std::collections::VecDeque;

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

#[derive(Clone)]
struct Edge{
    id : usize,
    v_from : usize,
    v_to : usize,
    length : usize,
    capacity : f64,
}

#[derive(Clone)]
struct Packet{
    id : usize,
    release_time : usize,
    path : Vec<usize>,
    entrance_time : usize,
    path_position : usize,
    edge_id : usize,
}

struct Network{
    n_vertices : usize,
    edges : Vec::<Edge>,
    edge_to_id : std::collections::HashMap::<(usize, usize), usize>,
    packets : Vec::<Packet>,
    queues : Vec<VecDeque<usize>>, // i-th queue corresponds to i-th edge
    time : usize,
    packets_arrived : usize,
    arrival_times : Vec<usize>,
}

impl Network{
    fn timestep(&mut self){
        // TODO: split into two parts: movement and queueing
        for packet_id in 0..self.packets.len(){
            //let &mut packet = &mut self.packets[packet_id];
            if self.packets[packet_id].release_time > self.time{
                // Packet not ready yet
                continue;
            }
            else if self.packets[packet_id].release_time == self.time{
                // Enter first edge on path
                self.packets[packet_id].entrance_time = self.time;
                self.packets[packet_id].path_position = self.packets[packet_id].path[0];
                let p_edge_id = self.get_edge_id(self.packets[packet_id].path[0], self.packets[packet_id].path[1]);
                self.packets[packet_id].edge_id = p_edge_id;               
                self.queues[p_edge_id].push_back(packet_id);
            }
            else if self.time == self.packets[packet_id].entrance_time + self.edges[self.packets[packet_id].edge_id].length{
                if self.packets[packet_id].path_position == self.packets[packet_id].path.len() - 1{
                    // packet at end of path
                    // TODO: save arrival time
                    self.packets[packet_id].path_position = std::usize::MAX;
                    self.packets[packet_id].edge_id = std::usize::MAX;
                    self.packets_arrived += 1;
                    self.arrival_times[packet_id] = self.time;
                }
                // packet changes into next edge
                let p_edge_id = self.get_edge_id(self.packets[packet_id].path[self.packets[packet_id].path_position], self.packets[packet_id].path[self.packets[packet_id].path_position + 1]);
                let popped_packet_id = self.queues[p_edge_id].pop_front().expect("pop_front() on empty queue called");
                self.queues[p_edge_id].push_back(popped_packet_id); // TODO: correct order
            }
            else{
                // Do nothing
            }
        }
        self.time += 1;
    }

    // Return: id of edge (v_from, v_to)
    fn get_edge_id(&mut self, v_from : usize, v_to : usize) -> usize{
        // TODO: implement
        *self.edge_to_id.get(&(v_from, v_to)).expect("edge doesn't exist")
    }
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n_vertices : usize = scan.next::<usize>();
    let n_edges : usize = scan.next::<usize>();
    let mut edges : Vec::<Edge> = Vec::<Edge>::new();
    let mut edge_to_id = std::collections::HashMap::<(usize, usize), usize>::new();
    for edge_id in 0..n_edges{
        let v_from = scan.next::<usize>();
        assert!(v_from < n_vertices, "vertex indices should be in [0, n_edges)");
        let v_to = scan.next::<usize>();
        assert!(v_to < n_vertices, "vertex indices should be in [0, n_edges)");
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
        edge_to_id.insert((v_from, v_to), edge_id);
    }

    let n_packets : usize = scan.next::<usize>();
    let mut packets = Vec::<Packet>::new();
    for packet_id in 0..n_packets{
        let release_time = scan.next::<usize>();
        let path_length = scan.next::<usize>();
        let path : Vec<usize> = (0..path_length).map(|_| scan.next::<usize>()).collect();
        assert!(path.len() >= 2, "paths should have length at least 2");
        packets.push(
            Packet{
                id : packet_id,
                release_time : release_time,
                path : path,
                entrance_time : std::usize::MAX, // default value
                path_position : std::usize::MAX, // default value
                edge_id : std::usize::MAX, // default value
            }
        );
    }
    let mut network = Network{
        n_vertices : n_vertices,
        edges : edges,
        edge_to_id : edge_to_id,
        packets : packets,
        queues : vec![VecDeque::new(); n_edges],
        time : 0,
        packets_arrived : 0,
        arrival_times : vec![n_packets; 0],
    };

    while network.packets_arrived < network.packets.len(){
        network.timestep();
    }

    write!(out, "Last arrival time:\n{}\n\nIndividual arrival times:", network.arrival_times.iter().max().expect("max failed somehow"));

    for arrival in network.arrival_times{
        write!(out, "{} ", arrival);
    }
}