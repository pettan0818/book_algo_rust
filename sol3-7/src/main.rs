use proconio::input;

fn main() {
    input! {
        K: i32,
        S: i32
    }
    println!("{}", search(K, S));
}

fn search(K: i32, S: i32) -> i32 {
    let mut res = 0;
    for X in 0..K + 1 {
        for Y in 0..K + 1 {
            if 0 <= S - X - Y && S - X - Y <= K {
                res = res + 1;
                // println!("{},{},{}", X, Y, S-X-Y);
            }
        }
    }
    res
}
