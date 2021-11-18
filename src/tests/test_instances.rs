// This file contains tests for the predefined instances defined under src/tests/instances/
#[cfg(test)]
use {
    std::collections::HashMap,
    crate::read_json,
    crate::write_json,
    crate::network::{Network, VertexId},
};


#[cfg(test)]
fn get_result_val(network : &Network, vertex_id_to_name : HashMap<VertexId, String>, instance_directory : &str) -> serde_json::Value{
    write_json::write_json(&network, vertex_id_to_name, &(instance_directory.to_owned() + "results.json"));
    let result_string = std::fs::read_to_string(&(instance_directory.to_owned() + "results.json")).unwrap();
    serde_json::from_str(&result_string).unwrap()
}

#[cfg(test)]
fn check_result_lengths(result_val : &serde_json::Value, n_packets : usize){
    assert_eq!(result_val["arrival_times"].as_array().unwrap().len(), n_packets);
    assert_eq!(result_val["travel_times"].as_array().unwrap().len(), n_packets);
    assert_eq!(result_val["commodity_ids"].as_array().unwrap().len(), n_packets);
}

#[test]
fn test_testing(){
    assert!(true);
    assert_eq!(2 + 2, 4);
    assert_ne!(2 + 2, 5);
}

#[test]
fn test_empty(){
    let instance_directory = "src/tests/instances/empty/";
    let (mut network, vertex_id_to_name) = read_json::read_jsons(&instance_directory);
    network.run_simulation();
    assert_eq!(network.vertices.len(), 0);
    assert_eq!(network.edges.len(), 0);
    assert_eq!(network.packets.len(), 0);
    assert_eq!(network.packets_arrived, 0);
    assert_eq!(network.arrival_times.len(), 0);
    assert_eq!(network.edge_queues.len(), network.edges.len());
    assert_eq!(network.time, 0);
    let result_val = get_result_val(&network, vertex_id_to_name, instance_directory);
    check_result_lengths(&result_val, network.packets.len());
}

#[test]
fn test_i_a1_b1(){ // flow starting at time 1 with rate 2
    let instance_directory = "src/tests/instances/i_a1_b1/";
    let (mut network, vertex_id_to_name) = read_json::read_jsons(&instance_directory);
    network.run_simulation();
    assert_eq!(network.vertices.len(), 2);
    assert_eq!(network.edges.len(), 1);
    assert_eq!(network.edges[0].length, 1);
    assert_eq!(network.packets.len(), 4);
    assert_eq!(network.packets_arrived, network.packets.len());
    assert_eq!(network.packets[0].release_time, 2);
    assert_eq!(network.packets[0].entrance_time.unwrap(), 2);
    assert_eq!(network.arrival_times.len(), network.packets.len());
    assert_eq!(network.arrival_times[0].unwrap(), 3);
    assert_eq!(network.edge_queues.len(), network.edges.len());
    assert_eq!(network.time, 6);
    let result_val = get_result_val(&network, vertex_id_to_name, instance_directory);
    check_result_lengths(&result_val, network.packets.len());
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[0], 3);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[1], 4);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[2], 5);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[3], 6);
}

#[test]
fn test_i_a0_5_b1(){ // flow starting at time 1 with rate 2
    let instance_directory = "src/tests/instances/i_a0,5_b1/";
    let (mut network, vertex_id_to_name) = read_json::read_jsons(&instance_directory);
    network.run_simulation();
    assert_eq!(network.vertices.len(), 2);
    assert_eq!(network.edges.len(), 1);
    assert_eq!(network.edges[0].length, 2);
    assert_eq!(network.packets.len(), 4);
    assert_eq!(network.packets_arrived, network.packets.len());
    assert_eq!(network.packets[0].release_time, 3); // (1 / 0.5) + 1
    assert_eq!(network.packets[0].entrance_time.unwrap(), 3); // (1 / 0.5) + 1
    assert_eq!(network.arrival_times.len(), network.packets.len());
    assert_eq!(network.arrival_times[0].unwrap(), network.packets[0].release_time + 2 + 1); // + 1 because capacity only 0.5
    assert_eq!(network.edge_queues.len(), network.edges.len());
    assert_eq!(network.time, 12); // 6 * (1 / 0.5)
    let result_val = get_result_val(&network, vertex_id_to_name, instance_directory);
    check_result_lengths(&result_val, network.packets.len());
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[0], 6); // 3 * (1 / 0.5)
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[1], 8);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[2], 10);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[3], 12);
}

#[test]
fn test_y_a1_b0_5(){
    let instance_directory = "src/tests/instances/y_a1_b0,5/";
    let (mut network, vertex_id_to_name) = read_json::read_jsons(&instance_directory);
    network.run_simulation();
    assert_eq!(network.vertices.len(), 4);
    assert_eq!(network.edges.len(), 3);
    assert_eq!(network.packets.len(), 2 * 9 * 2); // (#commodities) * (interval length) * (#packets per flow unit) 
    assert_eq!(network.packets_arrived, network.packets.len());
    assert_eq!(network.packets[0].release_time, 2);
    assert_eq!(network.packets[0].entrance_time.unwrap(), 3);
    assert_eq!(network.arrival_times.len(), network.packets.len());
    assert_eq!(network.arrival_times[0].unwrap(), 4); // 2 + 1 + 1
    assert_eq!(network.edge_queues.len(), network.edges.len());
    assert_eq!(network.time, network.arrival_times[0].unwrap() + (network.packets.len() / 2 - 1)); // 2 packets arrive per time unit 
    let result_val = get_result_val(&network, vertex_id_to_name, instance_directory);
    check_result_lengths(&result_val, network.packets.len());
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[0], 4);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[1], 4);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[2], 5);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[3], 6);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[9 * 2 + 0], 5);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[9 * 2 + 1], 6);
}

#[test]
fn test_zimmer(){
    let instance_directory = "src/tests/instances/zimmer/";
    let (mut network, vertex_id_to_name) = read_json::read_jsons(instance_directory);
    network.run_simulation();
    assert_eq!(network.vertices.len(), 6);
    assert_eq!(network.edges.len(), 5);
    assert_eq!(network.packets.len(), 9 * 2 + 16 * 4); // sum_i(interval length_i * rate_i)
    assert_eq!(network.packets_arrived, network.packets.len());
    assert_eq!(network.packets[0].release_time, 2);
    assert_eq!(network.packets[0].entrance_time.unwrap(), 4);
    assert_eq!(network.arrival_times.len(), network.packets.len());
    assert_eq!(network.arrival_times[0].unwrap(), 5); // 2 + 1 + 1 + 1
    assert_eq!(network.edge_queues.len(), network.edges.len());
    // The first 3 packets arrive alone, afterwards 2 packets arrive per time unit
    assert_eq!(network.time, network.arrival_times[0].unwrap() + (3 + ((network.packets.len() - 3) as f64 / 2.0).ceil() as usize) - 1);
    for arrival_time in network.arrival_times.clone(){
        assert_ne!(arrival_time, None);
    }
    let result_val = get_result_val(&network, vertex_id_to_name, instance_directory);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[0], 5);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[1], 6);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[2], 7);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[3], 9); // other commodity comes first
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[4], 10);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[5], 12); 
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[6], 13);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[7], 15);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[9 * 2 + 0], 8);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[9 * 2 + 1], 8);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[9 * 2 + 2], 9);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[9 * 2 + 3], 10);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[9 * 2 + 4], 11);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[9 * 2 + 5], 11);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[9 * 2 + 6], 12);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[9 * 2 + 7], 13);
    assert_eq!(result_val["arrival_times"].as_array().unwrap()[9 * 2 + 8], 14);
}