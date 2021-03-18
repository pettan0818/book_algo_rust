use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        step_cost: [i32; n]
    }

    //let n = 6;
    // let step_cost = vec![30, 10, 60, 10, 60, 50];

    let mut cost_result = vec![0];
    cost_result.extend(vec![99999; n - 1]);

    println!("{}", cost_explorer(step_cost, &mut cost_result));
}

fn cost_explorer(step_cost: Vec<i32>, cost_result: &mut Vec<i32>) -> i32 {
    for now in 0..cost_result.len() {
        if now + 1 < cost_result.len() {
            cost_result[now + 1] = min(
                cost_result[now] + (step_cost[now + 1] - step_cost[now]).abs(),
                cost_result[now + 1],
            );
        }
        if now + 2 < cost_result.len() {
            cost_result[now + 2] = cost_result[now] + (step_cost[now + 2] - step_cost[now]).abs();
        }
    }
    //println!("{:?}", cost_result);
    return cost_result[cost_result.len() - 1];
}
