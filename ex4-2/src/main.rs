fn main() {
    explorer(100);
}

fn explorer(depth: i32) -> i32 {
    // Report Call Another explorer.
    println!("Called depth in {}", depth);

    if depth == 0 {
        return 0;
    }
    let res = depth + explorer(depth - 1);
    println!("Sum: {} by {}", res, depth);

    return res;
}
