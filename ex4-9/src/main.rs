fn main() {
    let n: i64 = 49;
    let mut memo = vec![-1; (n + 1) as usize];
    let res = fibo(n, &mut memo);

    println!("Term {}: {}", n, res);
}

fn fibo(n: i64, memo: &mut Vec<i64>) -> i64 {
    // Report Call Another explorer.
    println!("Called fibo({})", n);
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    if memo[n as usize] != -1 {
        return memo[n as usize];
    }
    let res = fibo(n - 1, memo) + fibo(n - 2, memo);
    //println!("No.{}: {}", n, res);
    memo[n as usize] = res;
    res
}
