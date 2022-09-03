// Vectors - Resizable arrays

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Get single val
    println!("Single value: {}", numbers[0]);

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pops off last val
    numbers.pop();

    println!("{:?}", numbers);

    // Get array length
    println!("Vector length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers)); // note int = 4bytes

    // Get Slice
    let slice: &[i32] = &numbers[1..3]; // slice from 1..3
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Numbers: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}
