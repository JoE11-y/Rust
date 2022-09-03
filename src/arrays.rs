// Arrays - Fixed list where elements are the same data types

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Get single val
    println!("Single value: {}", numbers[0]);

    // Re-assign value
    numbers[2] = 20;

    // Get array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers)); // note int = 4bytes

    // Get Slice
    let slice: &[i32] = &numbers[1..3]; // slice from 1..3
    println!("Slice: {:?}", slice);
}
