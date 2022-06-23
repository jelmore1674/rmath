use std::env::args;
mod docs;
mod math;
use docs::docs::*;
use math::math::Args;
use math::math::*;

fn main() {
    let first_arg: Option<String> = args().nth(1);
    match first_arg {
        Some(first_argument) => match first_argument.as_str() {
            "help" | "-h" => print_docs(true),
            "-v" | "version" => println!("rmath 0.0.1"),
            _ => {
                let second_arg: Option<String> = args().nth(2);
                match second_arg {
                    Some(second_arg) => {
                        let third_arg: Option<String> = args().nth(3);
                        match third_arg {
                            Some(third_arg) => {
                                let percentage: Vec<&str> = first_argument.split("%").collect();
                                let args = Args {
                                    first_arg: percentage[0].to_string(),
                                    second_arg: second_arg,
                                    third_arg: third_arg,
                                };
                                math(args)
                            }
                            None => {
                                println!("Missing a number to do a math operation");
                                println!("");
                                println!("run help or -h for more information on how to use rmath")
                            }
                        }
                    }
                    None => {
                        println!("Must perform a math operation");
                        println!("");
                        print_math_operations();
                    }
                }
            }
        },
        None => print_docs(false),
    }
}
