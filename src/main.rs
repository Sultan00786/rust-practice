

fn pattern(n:&i8) -> () {

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


fn fib(n:&i8) -> i32 {
    if *n==0 {
        print!("{} ", 0);
        return 0;
    }
    else if *n==1 {
        print!("{} ", 1);
        return  1;
    }
    else {
        return fib(&(n-1))+fib(&(n-2));
    }
}


fn main() {
    println!("Hello, world!");
    let n:i8 = 6;
    pattern(&n);
    println!("\nFibnaci element at {}th positioin is {}", n, fib(&n));
}



