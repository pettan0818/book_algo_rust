use itertools::izip;
use proconio::input;

fn main() {
    input! {
        S: String,
    }
    println!("{}", search(S));
}

fn search(S: String) -> u64 {
    let mut res = 0;
    for cnt in 0..1 << (S.len() - 1) {
        // 2進数表現で、+を入れる入れないを管理する
        // 2進数表現に変換
        let bin = format!("{:0>width$b}", cnt, width = S.len() - 1);
        // Forループで扱えるようにVectorに変換
        let bin_vec: Vec<u64> = bin
            .chars()
            .map(|x| From::from(x.to_digit(10).unwrap())) // u64にぶち込む際にアップキャスト
            .collect();

        // 処理対象文字列をVectorに変換
        let mut target_vec: Vec<u64> = S
            .chars()
            .map(|x| From::from(x.to_digit(10).unwrap()))
            .collect();
        // 処理対象文字列の0文字目だけ取り出す
        let mut term = target_vec[0];
        target_vec = target_vec.drain(1..).collect();

        // println!("flags: {}", bin);

        for (flag, now_num) in izip!(bin_vec, target_vec) {
            if flag == 0 {
                term = term * 10 + now_num;
            }
            if flag == 1 {
                // println!("{}+", term);
                res = res + term;
                term = now_num;
            }
        }
        // println!("{}", term);
        res = res + term;
    }
    res
}
