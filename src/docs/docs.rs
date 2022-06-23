pub fn print_docs(add_operations: bool) {
    println!("Welcome to rmath, a simple terminal calculator");
    println!();
    println!("Usage:");
    println!("  rmath [options] [math operators] [numbers]");
    println!("");
    println!("Options:");
    println!("  -v,version      Print version information");
    println!("  -h, help        Show this help and see all available commands");
    println!("  any number,     Will do math operations");
    println!("");
    if add_operations {
        print_math_operations();
    }
}

pub fn print_math_operations() {
    println!("Math operations:");
    println!("  plus            Will add the two numbers together");
    println!("  minus           Will subtract the two numbers");
    println!("  times           Will multiply the two numbers");
    println!("  divide          Will divide the two numbers");
    println!("  of              Will find the percentage of the second number");
}
