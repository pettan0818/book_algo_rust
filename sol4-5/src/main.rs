use proconio::input;

fn main() {
    input! {
        max: i64,
    }
    let mut counter = 0;
    explorer(max, 0, 0b000, &mut counter);
    println!("{}", counter);
}

fn explorer(max_value: i64, cur_num: i64, flag: i64, counter: &mut i64) {
    if cur_num > max_value {
        return;
    }
    if flag & 0b111 == 0b111 {
        *counter = *counter + 1;
    }
    explorer(max_value, cur_num * 10 + 3, flag | 0b100, counter);
    explorer(max_value, cur_num * 10 + 5, flag | 0b010, counter);
    explorer(max_value, cur_num * 10 + 7, flag | 0b001, counter);
}
// use itertools::join;
// use permutator::Permutation;
// use std::collections::HashSet;

// fn main() {
//     let n: f32 = 1000000 as f32;
//     let x = n.log10() as i64 + 1;
//     println!("calc option: {}", x);
//     let r = make_data(x);
//     scan_from_result(r, n as i64);
// }

// fn scan_from_result(t: Vec<i64>, limit: i64) {
//     let x = t.iter().filter(|num| **num < limit);
//     println!("{}", x.clone().collect::<Vec<_>>().len());
//     println!("{:?}", x.clone().collect::<Vec<_>>());
// }

// fn make_data(figure_rank: i64) -> Vec<i64> {
//     let mut candidate = vec![vec![3, 5, 7]];
//     let mut candidate_list = vec![];
//     candidate_list.push(vec![3, 5, 7]);
//     let figure_rank: i64 = figure_rank - 3;

//     for _ in 0..figure_rank {
//         candidate = make_candidate(candidate);
//         candidate_list.extend(candidate.clone());
//     }

//     // println!("before::{:?}", candidate_list);

//     let mut final_res: HashSet<i64> = HashSet::new();

//     for comp in candidate_list {
//         let res = make_num_from_candidate(comp);
//         final_res.extend(res);
//     }
//     let mut checked: Vec<i64> = final_res.into_iter().collect();
//     checked.sort();
//     // println!("{:?}", checked);

//     checked
// }

// fn make_num_from_candidate(mut component: Vec<i64>) -> HashSet<i64> {
//     let mut res: Vec<i64> = Vec::new();
//     component.permutation().for_each(|p| {
//         let temp: i64 = join(&p, "").parse().unwrap();
//         res.push(temp);
//     });
//     let uniq: HashSet<i64> = res.into_iter().collect();
//     uniq
// }

// fn make_candidate(candidate: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
//     let mut res: Vec<Vec<i64>> = Vec::new();
//     for can in candidate {
//         for i in vec![3, 5, 7] {
//             let mut temp: Vec<i64> = vec![i];
//             temp.extend(can.iter().copied());
//             res.push(temp);
//         }
//     }
//     res
// }
