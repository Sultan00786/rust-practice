fn main() {
    let arr: Vec<i32> = [1, 2, 3, 4, 5, 6, 7, 8].to_vec();
    println!("{:?}", get_even_vec(arr));
}

fn get_even_vec(arr: Vec<i32>) -> Vec<i32> {
    let mut even_vec = Vec::new();

    for elem in arr {
        if elem & 1 == 0 {
            even_vec.push(elem);
        }
    }

    return even_vec;
}
