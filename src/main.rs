mod scanner;
mod network;
mod input;
mod fraction;
mod heap_element;
mod output;

fn main() {
    let mut network = input::input("src/instances/instance_l.txt");
    network.run_simulation();
    output::output_arrivals(&network);
}

#[test]
fn test_testing(){
    assert!(true);
    assert_eq!(2 + 2, 4);
    assert_ne!(2 + 2, 5);
}

#[test]
fn test_empty(){
    let mut network = input::input("src/instances/instance_empty.txt");
    network.run_simulation();
    assert_eq!(network.vertices.len(), 0);
    assert_eq!(network.edges.len(), 0);
    assert_eq!(network.packets.len(), 0);
    assert_eq!(network.packets_arrived, 0);
    assert_eq!(network.arrival_times.len(), 0);
    assert_eq!(network.edge_queues.len(), network.edges.len());
    assert_eq!(network.time, 0);
}

#[test]
fn test_i(){
    let mut network = input::input("src/instances/instance_i.txt");
    network.run_simulation();
    assert_eq!(network.vertices.len(), 2);
    assert_eq!(network.edges.len(), 1);
    assert_eq!(network.packets.len(), 1);
    assert_eq!(network.packets_arrived, 1);
    assert_eq!(network.arrival_times.len(), 1);
    assert_eq!(network.arrival_times[0].unwrap(), 1);
    assert_eq!(network.edge_queues.len(), network.edges.len());
    assert_eq!(network.time, 1);
}

#[test]
fn test_l(){
    let mut network = input::input("src/instances/instance_l.txt");
    network.run_simulation();
    assert_eq!(network.arrival_times.len(), 2);
    assert_eq!(network.arrival_times[0].unwrap(), 3);
    assert_eq!(network.arrival_times[1].unwrap(), 3);
    assert_eq!(network.time, 3);
}

#[test]
fn test_y(){
    let mut network = input::input("src/instances/instance_y.txt");
    network.run_simulation();
    assert_eq!(network.arrival_times[0].unwrap(), 7);
    assert_eq!(network.arrival_times[1].unwrap(), 8);
    assert_eq!(network.time, 8);
}