use std::collections::{BinaryHeap, VecDeque};

use crate::heap_element::HeapElement;
use crate::fraction::Fraction;

pub type Time = usize;
pub type VertexId = usize;
pub type EdgeId = usize;
pub type PacketId = usize;

#[derive(Clone)]
pub struct Vertex{
    pub incoming_edges : Vec<EdgeId>,
    pub outgoing_edges : Vec<EdgeId>,
}

#[derive(Clone)]
pub struct Edge{
    pub id : EdgeId,
    pub v_from : VertexId,
    pub v_to : VertexId,
    pub length : usize,
    pub capacity : f64,
}

#[derive(Clone)]
pub struct Packet{
    pub id : PacketId,
    pub release_time : Time,
    pub path : Vec<EdgeId>, // edges on path of packet
    pub entrance_time : Option<Time>,
    pub path_position : Option<usize>, // index in path : Vec<usize>
}

// TODO: separate Network into structs Network + State
pub struct Network{
    pub vertices : Vec<Vertex>,
    pub edges : Vec<Edge>,
    pub packets : Vec<Packet>,
    pub edge_queues : Vec<VecDeque<PacketId>>, // i-th queue corresponds to i-th edge
    pub leaving_queues : Vec<VecDeque<PacketId>>, 
    pub time : Time,
    pub packets_arrived : usize,
    pub arrival_times : Vec<Time>,
}

impl Network{

    pub fn run_simulation(&mut self){
        while self.packets_arrived < self.packets.len(){
            self.determine_leaving();
            self.node_transitions();
            self.packet_arrivals();
            self.timestep();
        }
    }

    // Determine the packets leaving the edges
    fn determine_leaving(&mut self){
        for edge_id in 0..self.edges.len(){
            // build buffer of candidate leaving packets for edge edge_id
            let mut buffer_queue = VecDeque::<PacketId>::new();
            loop{
                let edge_queue = &mut self.edge_queues[edge_id];
                if !edge_queue.is_empty(){
                    let front_packet = *edge_queue.front().unwrap();
                    let entrance_time = self.packets[front_packet].entrance_time.unwrap();
                    let leaving_time =  entrance_time + self.edges[edge_id].length;
                    if leaving_time <= self.time{
                        buffer_queue.push_back(front_packet);
                        edge_queue.pop_front();
                    }
                    else{
                        break;
                    }
                }
                else{
                    break;
                }
            }
            let leaving_queue = &mut self.leaving_queues[edge_id];
            leaving_queue.clear();

            while !buffer_queue.is_empty() && leaving_queue.len() + 1 <= self.edges[edge_id].capacity.floor() as usize { // TODO: changing capacity
                leaving_queue.push_back(*buffer_queue.front().unwrap());
                buffer_queue.pop_front();
            }
        }
    }

    // Determine transition of packets through nodes
    fn node_transitions(&mut self){
        for vertex in &self.vertices{
            for outgoing_edge_id in &vertex.outgoing_edges{
                // initialize priorities and queues of incoming arcs
                let mut incoming_queues = Vec::<(EdgeId, VecDeque<PacketId>)>::new();
                for incoming_edge_id in &vertex.incoming_edges{
                    let mut incoming_queue = VecDeque::<PacketId>::new();
                    let mut remaining_leaving_queue = VecDeque::<PacketId>::new();
                    // front-to-back iteration
                    for packet_id in &self.leaving_queues[*incoming_edge_id]{
                        let packet = &self.packets[*packet_id];
                        let next_position = packet.path_position.unwrap() + 1;
                        if packet.path.len() > next_position && packet.path[next_position] == *outgoing_edge_id{
                            incoming_queue.push_front(*packet_id);
                        }
                        else{
                            remaining_leaving_queue.push_front(*packet_id);
                        }
                    }
                    self.leaving_queues[*incoming_edge_id] = remaining_leaving_queue;
                    incoming_queues.push((*incoming_edge_id, incoming_queue));
                }
                // Add additional queue for packets entering network
                let mut entering_queue = VecDeque::<PacketId>::new();
                for (packet_id, packet) in self.packets.iter().enumerate(){
                    if packet.release_time == self.time && packet.path[0] == *outgoing_edge_id{
                        entering_queue.push_front(packet_id);
                        assert_eq!(self.packets[packet_id].path_position, None);
                    }
                }
                incoming_queues.push((EdgeId::MAX, entering_queue));


                // Build priority_queue for zipper method        
                let mut priority_queue = BinaryHeap::<HeapElement>::new();
                let mut original_queue_lengths = Vec::<usize>::new();
                for (queue_id, (incoming_edge, incoming_queue)) in incoming_queues.iter().enumerate(){
                    original_queue_lengths.push(incoming_queue.len());
                    if incoming_queue.len() > 0{
                        priority_queue.push(
                            HeapElement {
                                priority : Fraction::new(1, incoming_queue.len() as i64),
                                edge_id : *incoming_edge,
                                queue_id : queue_id,
                            }
                        );
                    }
                }
                while !priority_queue.is_empty(){
                    let top = priority_queue.peek().unwrap().clone();
                    assert!(top.priority <= Fraction::new(1, 1), "Error: priorities should be at most 1");
                    let incoming_queue = &mut incoming_queues[top.queue_id].1;
                    let packet_id = *incoming_queue.back().unwrap();
                    self.edge_queues[*outgoing_edge_id].push_front(packet_id);
                    self.packets[packet_id].entrance_time = Some(self.time);
                    incoming_queue.pop_back();

                    if incoming_queue.len() > 0{
                        let new_priority = Fraction::new(
                            (original_queue_lengths[top.queue_id] - incoming_queue.len() + 1) as i64,
                            original_queue_lengths[top.queue_id] as i64
                        );
                        priority_queue.push(
                            HeapElement{
                                priority : new_priority,
                                edge_id : top.edge_id,
                                queue_id : top.queue_id,
                            }
                        );
                        assert_ne!(new_priority, priority_queue.peek().unwrap().priority, "Error: new priority should be different")
                    }
                    priority_queue.pop();
                }
            }
        }
    }

    // Determine packets arrived at the last node of their path
    fn packet_arrivals(&mut self){
        for leaving_queue in &self.leaving_queues{
            for packet_id in leaving_queue{
                assert_eq!(self.packets[*packet_id].path_position.unwrap(), self.packets[*packet_id].path.len() - 1, "Error: packet should be at path end");
                assert_eq!(self.arrival_times[*packet_id], std::usize::MAX, "Error: packet should only arrive once");
                self.arrival_times[*packet_id] = self.time;
            }
        }
    }

    // Advance the time if not all packets arrived yet
    fn timestep(&mut self){
        if self.packets_arrived < self.packets.len(){
            self.time += 1;
        }
    }
}