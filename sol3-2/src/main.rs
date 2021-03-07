fn main() {
    let x = vec![100, 10, 20, 30, 40, 30, 1];
    println!("{}", search(x));
}

fn search(target_vec: Vec<i32>) -> i32 {
    // 2番目に小さな値を求める。
    let mut least = 9_999_999;
    let mut second_least = 9_999_998;
    let mut flag = false;
    for i in target_vec.iter() {
        if i < &least {
            if flag {
                second_least = least;
            }
            least = *i;
            flag = true;
            continue;
        }
        if i > &least && i < &second_least {
            second_least = *i;
        }
    }
    second_least
}
