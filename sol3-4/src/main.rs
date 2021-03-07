fn main() {
    let x = vec![100, 10, 20, 30, 40, 30, 1];
    let res = 99;

    println!("FINAL: {} / ANS: {}", search(x), res);
}

fn search(target_vec: Vec<i32>) -> i32 {
    // 最大/最小値を求める。
    let mut max = -1;
    let mut min = i32::MAX;

    for i in target_vec.iter() {
        if i < &min {
            min = *i;
        }
        if &max < i {
            max = *i
        }
        println!("MAX: {} \n MIN: {}\n==========", max, min);
    }
    max - min
}
