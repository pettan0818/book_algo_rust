use proconio::input;

fn main() {
    input! {
      n: i32,
      m: i32,
      c: i32,
      b_list: [i32; m],
      a_list: [[i32; m]; n]
    }

    let mut result_list = Vec::new();

    for a in a_list {
        let mut res = 0;
        for (a_t, b) in a.iter().zip(b_list.iter()) {
            res = res + a_t * b;
        }
        result_list.push(res + c);
    }
    println!("{:?}", result_list);
    let x: Vec<i32> = result_list.into_iter().filter(|&c| c > 0).collect();
    println!("{}", x.len());
}
