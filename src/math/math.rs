use crate::docs::docs;

pub struct Args {
    pub first_arg: String,
    pub second_arg: String,
    pub third_arg: String,
}

/// Take the args if a standard math problem is presented. Otherwise it will not run
/// The args will be passed through. If the arg is of it will get the percentage
pub fn math(args: Args) {
    match args.second_arg.as_str() {
        "plus" => {
            let math = add(
                args.first_arg.parse::<f64>().unwrap(),
                args.third_arg.parse::<f64>().unwrap(),
            );
            println!("{}", math);
        }
        "minus" => {
            let math = subtract(
                args.first_arg.parse::<f64>().unwrap(),
                args.third_arg.parse::<f64>().unwrap(),
            );
            println!("{}", math);
        }
        "divide" => {
            let math = divide(
                args.first_arg.parse::<f64>().unwrap(),
                args.third_arg.parse::<f64>().unwrap(),
            );
            println!("{}", math);
        }
        "times" => {
            let math = multiply(
                args.first_arg.parse::<f64>().unwrap(),
                args.third_arg.parse::<f64>().unwrap(),
            );
            println!("{}", math);
        }
        "of" => {
            let math = get_percentage(
                args.first_arg.parse::<f64>().unwrap(),
                args.third_arg.parse::<f64>().unwrap(),
            );
            println!("{}", math);
        }
        _ => {
            println!("Not a valid argument");
            println!();
            docs::print_math_operations();
        }
    }
}

fn add(x: f64, y: f64) -> f64 {
    return x + y;
}

fn subtract(x: f64, y: f64) -> f64 {
    return x - y;
}

fn divide(x: f64, y: f64) -> f64 {
    return x / y;
}

fn multiply(x: f64, y: f64) -> f64 {
    return x * y;
}

fn get_percentage(x: f64, y: f64) -> f64 {
    let percent = x / 100.0;
    return percent * y;
}
