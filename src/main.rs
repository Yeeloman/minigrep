// use std::io;
// fn main() {

//     let mut input: String = String::new();
//     io::stdin().read_line(&mut input)
//         .expect("Failed to read line");

//     let input: &str = input.trim();
//     let mut input = input.split_whitespace();
//     let searched_str: &str = input.next().expect("Failed to read line");

//     let file_name: &str = input.next().expect("Failed to read line");

// }


use std::{env, process};
use minigrep::Config;


fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    //one way to handle errors
    let conf: Config = Config::new(&args)
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(-1);
        });
    //another way to handle errors
    if let Err(e) = minigrep::run(conf){
            eprintln!("{}", e);
            process::exit(-1);
        };

    Ok(())
}
