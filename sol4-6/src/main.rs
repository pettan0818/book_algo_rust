fn main() {
    let components = vec![1, 2, 3, 4];
    let target: i32 = 6;
    let mut memo: Vec<Vec<i32>> = vec![vec![-1; 1 + target as usize]; components.len() + 1];

    if explorer(components.len(), &components, target, &mut memo) {
        println!("yes");
    } else {
        println!("no.");
    };
}

fn explorer(cur: usize, components: &Vec<i32>, target_sum: i32, memo: &mut Vec<Vec<i32>>) -> bool {
    println!("{} {}", cur, target_sum);
    if target_sum < 0 {
        return false;
    }
    if cur == 0 {
        if target_sum == 0 {
            return true;
        } else {
            return false;
        }
    }
    // Check my memo
    if memo[cur][target_sum as usize] != -1 {
        if memo[cur][target_sum as usize] == 1 {
            return true;
        } else {
            return false;
        }
    }

    // Select!
    if explorer(
        cur - 1,
        &components,
        target_sum - &components[cur - 1],
        memo,
    ) {
        memo[cur][target_sum as usize] = 1;
        return true;
    };
    if explorer(cur - 1, &components, target_sum, memo) {
        memo[cur][target_sum as usize] = 1;
        return true;
    };
    return false;
}
