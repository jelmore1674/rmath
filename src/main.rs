use std::env::args;
mod math;
use math::math::{*};
use math::math::Args;



fn main() {
    let first_argument: String = args().nth(1).unwrap();
    let second_argument: String = args().nth(2).unwrap();
    let third_argument: String = args().nth(3).unwrap();

    let percentage: Vec<&str> = first_argument.split("%").collect();

    let args = Args { 
        first_arg: percentage[0].to_string(), 
        second_arg:second_argument, 
        third_arg: third_argument
    };

    math(args)
    
}