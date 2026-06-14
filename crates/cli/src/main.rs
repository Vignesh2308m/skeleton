use std::env;
use std::fs::File;
use std::io::{BufReader, Read};


fn main()-> Result<(), std::io::Error>{
    // Getting input arguments
    let args:Vec<String> = env::args().collect();

    if args.len() != 3{
        panic!("number of argments mismatches");
    }

    let path = &args[1];
    let pattern = &args[2].as_bytes();

    //Loading File Buffer

    let file = File::open(path);
    if let Err(_) = file {
        panic!("Unable to Open File");
    }

    let mut buffer = BufReader::new(file.unwrap());

    let mut place_holder = [0; 1024];

    // let _ = buffer.read_to_string(&mut place_holder);

    loop {
        let n = buffer.read(&mut place_holder)?;

        if n == 0{
            break;
        }

        for i in 0..n{
            if &place_holder[i..i+pattern.len()] == *pattern {
                println!("Match found");
                break;
            }
        }
    }

    Ok(())
    // println!("{}", place_holder.contains(pattern));
}