fn main() {
    println!("Is Even: {}", is_even(40));
}

fn is_even(num: i32) -> bool {
    if num & 1 == 0 {
        return true;
    } else {
        return false;
    }
}
