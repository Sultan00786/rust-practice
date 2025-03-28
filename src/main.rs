
fn main() {
    println!("Hello, world!");
    pattern();
}

fn pattern() -> () {
    let n:i8 = 5;

    for i in 1..n+1 {
        for _j in 0..n-i{
            print!("* ");
        }
        for j in (1..i+1).rev().step_by(1){
            print!("{} ",j);
        }
        for j in (2..i+1).step_by(1){
            print!("{} ",j);
        }
        for _j in 0..n-i{
            print!("* ");
        }
        println!();
    }
}
