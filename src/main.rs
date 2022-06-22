use std::env::args;


fn main() {
    let first_argument: String = args().nth(1).unwrap();
    let second_argument: String = args().nth(2).unwrap();
    let third_argument: String = args().nth(3).unwrap();
    
    match second_argument.as_str() {
        "plus" | "+" =>  {
            let math: i32 = first_argument.parse::<i32>().unwrap() + third_argument.parse::<i32>().unwrap();
            println!("{}", math);
        }
         "minus" | "-" =>  {
            let math: i32 = first_argument.parse::<i32>().unwrap() - third_argument.parse::<i32>().unwrap();
            println!("{}", math);
        },
         "divide" | "/" =>  {
            let math: i32 = first_argument.parse::<i32>().unwrap() / third_argument.parse::<i32>().unwrap();
            println!("{}", math);
        },
         "multiply" | "times" | "*" =>  {
            let math: i32 = first_argument.parse::<i32>().unwrap() * third_argument.parse::<i32>().unwrap();
            println!("{}", math);
        }
        _ => println!("Not a valid argument"),
    }
}