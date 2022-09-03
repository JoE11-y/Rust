pub fn run() {
    //Print to console
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("{} is from {}", "Brad", "Mars");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Brad", "Mars", "code"
    );

    // Named Arguments
    println!(
        "{name} like to playe {activity}",
        name = "John",
        activity = "Baseball"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:0}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
