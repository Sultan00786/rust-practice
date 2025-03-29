fn reverse_arr(arr: &[i32; 5]) -> () {
    // .iter()	    Creates an iterator over the elements of an array.
    // .rev()	    Reverses the order of the elements in the iterator.

    let reversed_iter = arr.iter().rev(); // Creates a reversed iterator

    // {}    -->   Used for simple values (strings, integers)
    //             println!("Number: {}", 42);

    // {:?}  -->   Used for debugging (prints structs, arrays, etc.)
    //             println!("Array: {:?}", [1, 2, 3]);

    // {:#?} -->   Pretty-printing with indentation
    //             println!("Array: {:#?}", [1, 2, 3]);

    println!("{:?}", reversed_iter);
    // Pinrting array values using iterator
    for num in reversed_iter {
        print!("{} ", num); // Prints numbers in reverse order
    }

    // .cloned()	Creates a copy of each element (used when dealing with references).
    // .collect()	Converts the iterator back into a collection (e.g., a Vec<T>).

    let reversed: Vec<i32> = arr.iter().rev().cloned().collect(); // Clones each value

    println!("\n{:#?}", reversed);
}

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

    reverse_arr(&xs);
}
