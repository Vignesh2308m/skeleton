use std::env;
use std::fs::File;
use std::io::{BufReader, Read};


fn main(){
    // Getting input arguments
    let args:Vec<String> = env::args().collect();

    if args.len() != 3{
        panic!("number of argments mismatches");
    }

    let path = &args[1];
    let pattern = &args[2];

    //Loading File Buffer

    let file = File::open(path);
    if let Err(_) = file {
        panic!("Unable to Open File");
    }

    let mut buffer = BufReader::new(file.unwrap());

    let mut place_holder = String::new();

    let _ = buffer.read_to_string(&mut place_holder);

    println!("{}", place_holder.contains(pattern));
}