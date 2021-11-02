use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args : Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // let query = &args[1];
    // let filename = &args[2];

    let curr_config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for this query: {}", &curr_config.query);
    // println!("File to be searched: {}", &curr_config.filename);

    // run(curr_config);
    // if let Err(e) = run(curr_config) {
    //     println!("Application error : {}", e);
    //     process::exit(1);
    // }   
    if let Err(e) = minigrep::run(curr_config) {
        eprintln!("Application error : {}", e);
        process::exit(1);
    }
}