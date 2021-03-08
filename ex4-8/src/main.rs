fn main() {
    let n = 6;
    let res = fibo(n);
    println!("Term {}: {}", n, res);
}

fn fibo(n: i32) -> i32 {
    // Report Call Another explorer.
    //println!("Called fibo({})", n);
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let res = fibo(n - 1) + fibo(n - 2);
    //println!("No.{}: {}", n, res);
    res
}
