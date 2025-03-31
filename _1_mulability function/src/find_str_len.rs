fn main() {
    println!("String length is : {}", str_len("Hellow"));
}

fn str_len(str: &str) -> i32 {
    let mut count: i32 = 0;
    for _chr in str.to_string().chars() {
        count += 1;
    }
    return count;
}
