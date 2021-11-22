use std::env;
mod network;
mod fraction;
mod heap_element;
mod read_json;
mod write_json;
mod tests;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        // no arguments passed
        1 => {
            println!("Please pass directory where packet routing instance is located");
        },
        // one argument passed
        2 => {
            let instance_directory : String = args[1].parse().unwrap();
            let (mut network, vertex_id_to_name) = read_json::read_jsons(&instance_directory);
            network.run_simulation();
            write_json::write_json(&network, vertex_id_to_name, &(instance_directory.to_owned() + "results.json"));
        }
        // too many arguments passed
        _ => {
            println!("Too many arguments passed");
        }
    }
}