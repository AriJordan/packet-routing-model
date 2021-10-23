use std::collections::{BinaryHeap, VecDeque};

use crate::heap_element::MaxHeapElement;
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
    pub average_capacity : Fraction,
    pub current_capacity : Fraction,
}

#[derive(Clone)]
pub struct Packet{
    pub id : PacketId,
    pub release_time : Time,
    pub path : Vec<EdgeId>, // edges on path of packet
    pub entrance_time : Option<Time>,
    pub path_position : Option<usize>, // index in path : Vec<usize>
}

// TODO: separate Network into structs Network + State?
pub struct Network{
    pub vertices : Vec<Vertex>,
    pub edges : Vec<Edge>,
    pub packets : Vec<Packet>,
    pub edge_queues : Vec<VecDeque<PacketId>>, // i-th queue corresponds to i-th edge
    pub leaving_queues : Vec<VecDeque<PacketId>>, 
    pub time : Time,
    pub packets_arrived : usize,
    pub arrival_times : Vec<Option<Time>>,
}

impl Network{

    pub fn run_simulation(&mut self){
        while self.packets_arrived < self.packets.len(){
            #[cfg(debug_assertions)]
            println!("Time: {}", self.time);
            self.determine_leaving();
            #[cfg(debug_assertions)]
            println!("Starting node transitions");
            self.node_transitions();
            #[cfg(debug_assertions)]
            println!("Starting packet arrivals");
            self.packet_arrivals();
            #[cfg(debug_assertions)]
            println!("#Packets arrived: {}", self.packets_arrived);
            self.timestep();
        }
    }

    // Determine the packets leaving the edges
    fn determine_leaving(&mut self){
        for edge_id in 0..self.edges.len(){
            // build buffer of candidate leaving packets for edge edge_id
            let mut buffer_queue = VecDeque::<PacketId>::new();
            let edge_queue = &mut self.edge_queues[edge_id];
            for packet_id in edge_queue.iter_mut(){
                let entrance_time = self.packets[*packet_id].entrance_time.unwrap();
                let leaving_time = entrance_time + self.edges[edge_id].length;
                if leaving_time <= self.time{
                    buffer_queue.push_back(*packet_id);
                }
                else{
                    break;
                }
            }

            let leaving_queue = &mut self.leaving_queues[edge_id];
            assert!(leaving_queue.is_empty());
            loop{
                let n_leaving = Fraction{numerator: (leaving_queue.len() + 1) as i64, denominator : 1};
                if buffer_queue.is_empty(){
                    break;
                }
                if n_leaving > self.edges[edge_id].current_capacity{
                    break;
                }
                leaving_queue.push_back(*buffer_queue.front().unwrap());
                buffer_queue.pop_front();
                edge_queue.pop_front();
            }
            let avg_cap = self.edges[edge_id].average_capacity.clone();
            let cur_cap = &mut self.edges[edge_id].current_capacity;   
            if buffer_queue.is_empty(){ // |B_e(t - 1)| <= v^_e(t - 1)
                *cur_cap = avg_cap;
            }
            else{
                *cur_cap = avg_cap + *cur_cap - (*cur_cap).floor();
            }
        }
    }

    // Determine transition of packets through nodes
    fn node_transitions(&mut self){
        for vertex in &self.vertices{
            // TODO: improve runtime here?
            for outgoing_edge_id in &vertex.outgoing_edges{
                #[cfg(debug_assertions)]
                println!("Considering outgoing_edge_id {}", outgoing_edge_id);
                // initialize priorities and queues of incoming arcs
                let mut incoming_queues = Vec::<(EdgeId, VecDeque<PacketId>)>::new();
                for incoming_edge_id in &vertex.incoming_edges{
                    let mut incoming_queue = VecDeque::<PacketId>::new();
                    let mut remaining_leaving_queue = VecDeque::<PacketId>::new();
                    // front-to-back iteration
                    for packet_id in &self.leaving_queues[*incoming_edge_id]{
                        let packet = &mut self.packets[*packet_id];
                        let next_position = packet.path_position.unwrap() + 1;
                        #[cfg(debug_assertions)]
                        println!("Packet {} path.len(): {}, path[next]: {}", packet_id, packet.path.len(), packet.path[next_position]);
                        if packet.path.len() > next_position && packet.path[next_position] == *outgoing_edge_id{
                            // TODO: Test order!
                            incoming_queue.push_back(*packet_id);
                            packet.path_position = Some(next_position);
                            #[cfg(debug_assertions)]
                            println!("Packet {} has new path_position {}", packet_id, packet.path_position.unwrap());
                        }
                        else{
                            // TODO: Test order!
                            remaining_leaving_queue.push_back(*packet_id);
                        }
                    }
                    self.leaving_queues[*incoming_edge_id] = remaining_leaving_queue;
                    incoming_queues.push((*incoming_edge_id, incoming_queue));
                }
                // Add additional queue for packets entering network
                let mut entering_queue = VecDeque::<PacketId>::new();
                // TODO: runtime?
                for (packet_id, packet) in self.packets.iter_mut().enumerate(){
                    if packet.release_time == self.time && packet.path[0] == *outgoing_edge_id{
                        #[cfg(debug_assertions)]
                        println!("Packet {} enters network, entering_queue", packet_id);
                        entering_queue.push_back(packet_id);
                        assert_eq!(packet.path_position, None);
                        #[cfg(debug_assertions)]
                        println!("Packet {} has new path_position {}", packet_id, 0);
                        packet.path_position = Some(0);
                    }
                }
                incoming_queues.push((EdgeId::MAX, entering_queue));


                // Build priority_queue for zipper method   
                let mut priority_queue = BinaryHeap::<MaxHeapElement>::new();
                let mut original_queue_lengths = Vec::<usize>::new();
                for (queue_id, (incoming_edge_id, incoming_queue)) in incoming_queues.iter().enumerate(){
                    original_queue_lengths.push(incoming_queue.len());
                    if incoming_queue.len() > 0{
                        priority_queue.push(
                            MaxHeapElement {
                                priority : Fraction::new(1, original_queue_lengths[queue_id] as i64),
                                edge_id : *incoming_edge_id,
                                queue_id : queue_id,
                            }
                        );
                    }
                }
                #[cfg(debug_assertions)]
                println!("priority_queue length: {}", priority_queue.len());
                while !priority_queue.is_empty(){
                    let top = priority_queue.pop().unwrap();
                    assert!(top.priority <= Fraction::new(1, 1), "Error: priorities should be at most 1");
                    let incoming_queue = &mut incoming_queues[top.queue_id].1;
                    // TODO: test
                    let packet_id = incoming_queue.pop_front().unwrap();
                    #[cfg(debug_assertions)]
                    println!("Packet {} enters edge_queue", packet_id);
                    // TODO: test
                    self.edge_queues[*outgoing_edge_id].push_back(packet_id);
                    
                    self.packets[packet_id].entrance_time = Some(self.time);
                
                    if incoming_queue.len() > 0{
                        let new_priority = Fraction::new(
                            (original_queue_lengths[top.queue_id] - incoming_queue.len() + 1) as i64,
                            original_queue_lengths[top.queue_id] as i64
                        );
                        priority_queue.push(
                            MaxHeapElement{
                                priority : new_priority,
                                edge_id : top.edge_id,
                                queue_id : top.queue_id,
                            }
                        );
                    }
                    
                }
            }
        }
    }

    // Determine packets arrived at the last node of their path
    fn packet_arrivals(&mut self){
        for leaving_queue in &self.leaving_queues{
            #[cfg(debug_assertions)]
            println!("{} packets leaving from leaving_queue", leaving_queue.len());
            for packet_id in leaving_queue{
                assert_eq!(self.packets[*packet_id].path_position.unwrap(), self.packets[*packet_id].path.len() - 1, "Error: packet should be at path end");
                assert_eq!(self.arrival_times[*packet_id], None, "Error: packet should only arrive once");
                #[cfg(debug_assertions)]
                println!("Packet {} has arrived", *packet_id);
                self.arrival_times[*packet_id] = Some(self.time);
                self.packets_arrived += 1;
            }
        }
        for leaving_queue in &mut self.leaving_queues {
            leaving_queue.clear();
        }
    }

    // Advance the time if not all packets arrived yet
    fn timestep(&mut self){
        if self.packets_arrived < self.packets.len(){
            self.time += 1;
        }
    }
}