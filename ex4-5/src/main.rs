fn main() {
    let res = gcd(15, 51);
    assert_eq!(res, 3);
    println!("{}", res);
}

fn gcd(m: i32, n: i32) -> i32 {
    // Report Call Another explorer.
    let mod_both = m % n;
    if mod_both == 0 {
        return n;
    }
    gcd(n, mod_both)
}
