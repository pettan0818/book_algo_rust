use std::io;

fn main() {
    // Get Value Length.
    println!("Please input vector length...");
    let num = get_from_stdin();

    // Get Desired number.
    println!("Please Input target Number...");
    let target = get_from_stdin();

    println!("Please input your vector. one line per one value...");
    let mut target_vector = Vec::new();

    for _c in 0..num {
        let x = get_from_stdin();
        target_vector.push(x);
    }

    // Linear Search.
    let mut index_exsit :usize = usize::MAX;
    for (i, t) in target_vector.iter().enumerate(){
        if *t == target {  // 参照は解決して比較。
            index_exsit = i;
        }
    }

    // Presentation Result.
    if index_exsit == usize::MAX {
        println!("Not Found.");
    }
    else{
        println!("Found at {}", index_exsit)
    }
}

fn get_from_stdin() -> i32 {
    let mut n = String::new();
    io::stdin()
    .read_line(&mut n)
    .expect("ERR: Failed load value.");
    let num: i32 = match n.trim().parse(){
        Ok(num) => num,
        Err(_) => panic!("This is not a number: {}", n)
    };
    num
}
