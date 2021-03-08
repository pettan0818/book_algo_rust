fn main() {
    let n: i64 = 4;
    let v: Vec<i64> = vec![1, 2, 3, 4];
    let w: i64 = 11;

    if explorer(n, &v, w) {
        println!("TRUE");
    } else {
        println!("FALSE");
    }
}

fn explorer(lasting_count: i64, target_vector: &Vec<i64>, target_result: i64) -> bool {
    // Report Call Another explorer.
    if lasting_count == 0 {
        if target_result == 0 {
            return true;
        }
        return false;
    }
    // Select target_vector[lasting_count - 1]
    if explorer(
        lasting_count - 1,
        &target_vector,
        target_result - target_vector[(lasting_count - 1) as usize],
    ) {
        return true;
    }
    // NOT Select target_vector[lasting_count - 1]
    if explorer(lasting_count - 1, &target_vector, target_result) {
        return true;
    }
    return false;
}
