use std::collections::VecDeque;

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
    id : EdgeId,
    v_from : VertexId,
    v_to : VertexId,
    length : usize,
    capacity : f64,
}

#[derive(Clone)]
pub struct Packet{
    id : PacketId,
    release_time : Time,
    path : Vec<EdgeId>, // edges on path of packet
    entrance_time : Option<Time>,
    path_position : Option<usize>, // index in path : Vec<usize>
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
            self.node_transition();
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
    fn node_transition(&mut self){
        for (vertex_id, vertex) in self.vertices.iter().enumerate(){
            for outgoing_edge_id in vertex.outgoing_edges{
                // initialize priorities and queues of incoming arcs
                let incoming_queues = Vec::<VecDeque<PacketId>>::new();
                for incoming_edge_id in vertex.incoming_edges{
                    let mut incoming_queue = VecDeque::<PacketId>::new();
                    let mut remaining_leaving_queue = VecDeque::<PacketId>::new();
                    // front-to-back iteration
                    for packet_id in self.leaving_queues[incoming_edge_id]{
                        let packet = &self.packets[packet_id];
                        if packet.path.len() > packet.path_position.unwrap() + 1 && packet.path[packet.path_position + 1] == outgoing_edge_id{
                            incoming_queue.push_front(packet_id);
                        }
                        else{
                            remaining_leaving_queue.push_front(packet_id);
                        }
                    }
                    self.leaving_queues[incoming_edge_id] = remaining_leaving_queue;
                    incoming_queues.push(incoming_queue);
                }
            }
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
                                println!("self.time: {}", self.time);
                                println!("Packet {} entrance_time: {}", packet.id, packet.entrance_time.unwrap());
                                println!("entrance_time + edge length: {}", entrance_time +  self.edges[edge_id].length);
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
                                        packet.entrance_time = Some(self.time);
                                        println!("packet {} now has path position {}", packet.id, packet.path_position.unwrap());
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