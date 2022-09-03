// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Joe";

    let mut age = 37; // to make var mutable add mut

    println!("My name is {} and I am {}", name, age);

    age = 28;

    println!("My name is {} and I am {}", name, age);

    // Define constants
    // explicity define type and use uppercase for names
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Joe", 27);

    println!("{} is {}", my_name, my_age)
}
