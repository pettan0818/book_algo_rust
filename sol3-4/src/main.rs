fn main() {
    let x = vec![100, 10, 20, 30, 40, 30, 1];
    let res = 10;

    println!("FINAL: {} / ANS: {}", search(x), res);
}

fn search(target_vec: Vec<i32>) -> i32 {
    // 2番目に小さな値を求める。
    let max = i32::MAX;
    let mut least = max;
    let mut second_least = max;
    for i in target_vec.iter() {
        if i < &least {
            second_least = least;
            least = *i;
        } else if i < &second_least {
            second_least = *i;
        }
        println!(
            "least: {} \n second_least: {}\n==========",
            least, second_least
        );
    }
    second_least
}
