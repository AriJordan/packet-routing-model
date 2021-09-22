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
    n_edges : usize,
    edges : Vec::<Edge>,
    n_packets : usize,
    packets : Vec::<Packet>,
    queues : Vec<VecDeque<usize>>, // i-th queue corresponds to i-th edge
    time : usize,
    packets_arrived : bool,
}

impl Network{
    fn timestep(&mut self){
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
    }

    // Return: id of edge (v_from, v_to)
    fn get_edge_id(&mut self, v_from : usize, v_to : usize) -> usize{
        // TODO: implement
        0
    }
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n_vertices : usize = scan.next::<usize>();
    let n_edges : usize = scan.next::<usize>();
    let mut edges : Vec::<Edge> = Vec::<Edge>::new();
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
    let network = Network{
        n_vertices : n_vertices,
        n_edges : n_edges,
        edges : edges,
        n_packets : n_packets,
        packets : packets,
        queues : vec![VecDeque::new(); n_edges],
        time : 0,
        packets_arrived : false,
    };
}