use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let iter = s.split_whitespace().map(|i| i.parse::<i32>().unwrap());
    let res = search(iter.collect());

    //println!("FINAL: {} / ANS: {}", search(x), res);
    println!("{}", res);
}

fn search(target_vec: Vec<i32>) -> i32 {
    // 2進法でいくと10000だと4、10100だと2になる。
    let mut res = 99;

    for i in target_vec.iter() {
        let latest_res = shift_num(*i);
        if res > latest_res {
            res = latest_res;
        }

        //println!("NOW: {} ==========", res);
    }
    res
}

fn shift_num(target: i32) -> i32 {
    let mut target_num = target;
    let mut res: i32 = 0;
    while target_num % 2 == 0 {
        res = res + 1;
        target_num = target_num / 2;
    }
    res
}
