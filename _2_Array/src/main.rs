

fn main() {
    // println!("Hellow world");
    // let array = [10, 30, 60, 100];
    // print!("{}",array[2])

    // Arrays in Rust are declared using brackets [], where the type and length are part of the type signature [T; length].
    let xs: [i32; 5] = [5, 55, 66, 7, 2];
    let ys: [i32; 9] = [5, 55, 66, 7, 2, 55, 66, 7, 2];

    println!("First element of the array: {}", xs[0]); // Outputs: First element of the array: 1

    println!("Last element of the array: {}", xs[1]); // Outputs: Second element of the array: 2

    // Outputs: Number of elements in array: 5
    println!("Number of elements in array: {}", xs.len());
    // Outputs: Number of elements in array: 9
    println!("Number of elements in array: {}", ys.len());

    println!("Array occupies {} bytes", size_of_val(&xs)); // Outputs the size in bytes occupied by `xs`
    println!("Array occupies {} bytes", size_of_val(&ys)); // Outputs the size in bytes occupied by `ys`
}
