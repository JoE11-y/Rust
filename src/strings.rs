// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure -- Use when you need to modify or own string data

pub fn run() {
    let hello = "Hello"; // immutable str

    let mut hello2 = String::from("Hello "); // modifiable String

    // Get length
    println!("Length mutable: {}, Length Immutable: {}", hello2, hello);

    // Push char
    hello2.push('W');

    // Push string
    hello2.push_str("orld");

    println!("{}", hello2);

    // Capacity in bytes
    println!("Capacity: {}", hello2.capacity());

    // is Empty
    println!("Is Empty: {}", hello2.is_empty());

    // Contains
    println!("Contains 'World' {}", hello2.contains("World"));

    // Replace
    println!("Replace: {}", hello2.replace("World", "There"));
    println!("{}", hello2);

    // Loop through stirng by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len()); // only panicks when left != right
    assert_eq!(10, s.capacity());
}
