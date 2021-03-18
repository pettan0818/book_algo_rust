fn main() {}
// use itertools::join;
// use permutator::Permutation;
// use std::collections::HashSet;

// fn main() {
//     let n: f32 = 1000000 as f32;
//     let x = n.log10() as i32 + 1;
//     println!("calc option: {}", x);
//     let r = make_data(x);
//     scan_from_result(r, n as i32);
// }

// fn scan_from_result(t: Vec<i32>, limit: i32) {
//     let x = t.iter().filter(|num| **num < limit);
//     println!("{}", x.clone().collect::<Vec<_>>().len());
//     println!("{:?}", x.clone().collect::<Vec<_>>());
// }

// fn make_data(figure_rank: i32) -> Vec<i32> {
//     let mut candidate = vec![vec![3, 5, 7]];
//     let mut candidate_list = vec![];
//     candidate_list.push(vec![3, 5, 7]);
//     let figure_rank: i32 = figure_rank - 3;

//     for _ in 0..figure_rank {
//         candidate = make_candidate(candidate);
//         candidate_list.extend(candidate.clone());
//     }

//     // println!("before::{:?}", candidate_list);

//     let mut final_res: HashSet<i32> = HashSet::new();

//     for comp in candidate_list {
//         let res = make_num_from_candidate(comp);
//         final_res.extend(res);
//     }
//     let mut checked: Vec<i32> = final_res.into_iter().collect();
//     checked.sort();
//     // println!("{:?}", checked);

//     checked
// }

// fn make_num_from_candidate(mut component: Vec<i32>) -> HashSet<i32> {
//     let mut res: Vec<i32> = Vec::new();
//     component.permutation().for_each(|p| {
//         let temp: i32 = join(&p, "").parse().unwrap();
//         res.push(temp);
//     });
//     let uniq: HashSet<i32> = res.into_iter().collect();
//     uniq
// }

// fn make_candidate(candidate: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//     let mut res: Vec<Vec<i32>> = Vec::new();
//     for can in candidate {
//         for i in vec![3, 5, 7] {
//             let mut temp: Vec<i32> = vec![i];
//             temp.extend(can.iter().copied());
//             res.push(temp);
//         }
//     }
//     res
// }
