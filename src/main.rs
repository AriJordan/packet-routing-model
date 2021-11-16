mod network;
mod fraction;
mod heap_element;
mod read_json;
mod write_json;
mod tests;

fn main() {
    let instance_directory = read_json::get_instance_directory();
    let (mut network, vertex_id_to_name) = read_json::read_jsons(&instance_directory);
    network.run_simulation();
    write_json::write_json(&network, vertex_id_to_name, &(instance_directory.to_owned() + "results.json"));
}