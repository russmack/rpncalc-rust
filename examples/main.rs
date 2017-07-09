extern crate rpncalc;

use std::env;
use std::process;

fn usage() {
    println!(r#"Usage rpncalc-rust "182 7 27 2 3 4 5 + + + - * /""#);
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
    }

    println!("Calculating: {}", args[1].trim());

    match rpncalc::calculate(&args[1]) {
        Ok(n) => println!("Result: {}", n),
        Err(e) => println!("{}", e),
    };
}
