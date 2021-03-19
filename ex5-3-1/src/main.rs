use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        k: usize,
        step_cost: [i32; n]
    }

    // let n = 10;
    // let k = 4;
    // let step_cost = vec![40, 10, 20, 70, 80, 10, 20, 70, 80, 60];

    let mut cost_result = vec![0];
    cost_result.extend(vec![std::i32::MAX; n - 1]);

    println!("{}", cost_explorer(k, &step_cost, &mut cost_result));
}

fn cost_explorer(limit: usize, step_cost: &Vec<i32>, cost_result: &mut Vec<i32>) -> i32 {
    for now in 0..cost_result.len() {
        checker(now, limit, step_cost, cost_result);
    }
    // println!("{:?}", cost_result);
    return cost_result[cost_result.len() - 1];
}

fn checker(now: usize, limit: usize, step_cost: &Vec<i32>, cost_result: &mut Vec<i32>) {
    for i in 1..1 + limit {
        if now + i < cost_result.len() {
            cost_result[now + i] = min(
                cost_result[now] + (step_cost[now + i] - step_cost[now]).abs(),
                cost_result[now + i],
            );
        }
    }
}
