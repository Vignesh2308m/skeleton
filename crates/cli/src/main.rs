use std::fs;


fn main() {
    let file = fs::read_to_string("./data/intro.txt");
    if let Err(err) = file {
        println!("{}", err.to_string());
        return
    }

    println!("{}",file.ok().unwrap());
}