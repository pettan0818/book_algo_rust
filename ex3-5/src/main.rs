fn main() {
    let n: i32 = 5;
    let w = 25;
    let n_vec: Vec<i32> = vec![10, 5, 3, 8, 7];
    assert_eq!(n, (n_vec.len() as i32)); // lenはusizeを返すのでキャスト
    println!("Input Info\nN:{}, W:{}, NVec: {:?}", n, w, n_vec);

    let res: bool = algo(n, w, n_vec);

    if !res {
        println!("Not Found On this condtion...");
    }
}

fn algo(n: i32, w: i32, n_vec: Vec<i32>) -> bool {
    // exsit: found or not
    let mut exist: bool = false;
    for bit in 0..(1 << n) {
        // bit shows 0-2^length(n_vec)
        //println!("{:0>width$b}", bit, width = (n as usize)); // https://qiita.com/gyu-don/items/5e71dcc14ffb4b424e80

        let mut sum: i32 = 0; // ここに足し込む
        let mut using_vec: Vec<i32> = Vec::new();
        for fig in 0..n {
            if bit & (1 << fig) != 0 {
                // bit(0001) & (0001) -> 0001 != 0 <True> // bit(0001) & (0000) -> 0000 != 0 <false>  // Int -> boolは比較するしかない
                sum = sum + n_vec[fig as usize];
            }

            if w == sum {
                exist = true;
                for fig in 0..n {
                    if bit & (1 << fig) != 0 {
                        using_vec.push(n_vec[fig as usize]);
                    }
                }
                println!(
                    "Found combination...On{:0>width$b}",
                    bit,
                    width = n as usize
                );
                println!("by using {:?}", using_vec);
                return exist;
            }
        }
    }
    return exist;
}
