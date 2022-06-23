use std::env::args;
mod math;
use math::math::Args;
use math::math::*;

fn main() {
    let first_arg: Option<String> = args().nth(1);
    match first_arg {
        Some(first_argument) => match first_argument.as_str() {
            "help" | "-h" => println!("Will print docs"),
            "-v" | "version" => println!("rmath 0.1"),
            _ => {
                let second_argument: String = args().nth(2).unwrap();
                let third_argument: String = args().nth(3).unwrap();
                let percentage: Vec<&str> = first_argument.split("%").collect();
                let args = Args {
                    first_arg: percentage[0].to_string(),
                    second_arg: second_argument,
                    third_arg: third_argument,
                };
                math(args)
            }
        },
        None => println!("Will pring future docs"),
    }
}
