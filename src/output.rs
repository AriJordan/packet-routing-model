// use std::io::{BufWriter, stdout, Write};
use crate::network;

pub fn output_arrivals(network : &network::Network, ){
    //let out = &mut BufWriter::new(stdout());
    println!("Last arrival time:\n{}\n\nIndividual arrival times:", network.arrival_times.iter().max().unwrap().expect("max failed somehow"));
    for arrival in &network.arrival_times{
        print!("{} ", arrival.unwrap());
    }
}