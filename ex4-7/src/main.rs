fn main() {
    let mut result_vec: Vec<i64> = vec![0, 1];
    let target = 50;

    for i in 2..target {
        result_vec.push(result_vec[i - 1] + result_vec[i - 2]);
    }
    println!("{}", result_vec.last().unwrap());
}
