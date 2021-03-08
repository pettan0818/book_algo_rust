use itertools::izip;
use proconio::input;

fn main() {
    input! {
        s: String,
    }
    println!("{}", search(s));
}

fn search(s: String) -> u64 {
    let mut res = 0;
    for cnt in 0..1 << (s.len() - 1) {
        // 2進数表現で、+を入れる入れないを管理する
        // 2進数表現に変換
        let bin = format!("{:0>width$b}", cnt, width = s.len() - 1);
        // Forループで扱えるようにVectorに変換
        let bin_vec: Vec<u64> = bin
            .chars() // String->Iter(char)
            .map(|x| From::from(x.to_digit(10).unwrap())) // u64にぶち込む際にアップキャスト<From::from>
            .collect();

        // 処理対象文字列をVectorに変換
        let mut target_vec: Vec<u64> = s
            .chars()
            .map(|x| From::from(x.to_digit(10).unwrap())) // to_digitでChar->u32
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
                term = now_num; // 処理中の数字を次の項にしてあげる必要がある。
            }
        }
        // println!("{}", term);
        res = res + term; //最期まで残った項を出力結果に加算
    }
    res
}
