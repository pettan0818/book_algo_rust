use std::collections::HashMap;

fn main() {
    let n: i128 = 101;

    let mut memo_map: HashMap<i128, i128> = HashMap::new();
    memo_map.insert(0, 0);
    memo_map.insert(1, 0);
    memo_map.insert(2, 1);

    tribo(n, &mut memo_map);

    for i in 0..n + 1 {
        println!("{}: {}", i, memo_map[&i]);
    }
}

fn tribo(target: i128, memo: &mut HashMap<i128, i128>) -> i128 {
    // Init.
    if memo.contains_key(&target) {
        return memo[&target];
    }

    // calc
    let x: i128 = tribo(target - 1, memo) + tribo(target - 2, memo) + tribo(target - 3, memo);

    memo.insert(target, x);

    x
}
