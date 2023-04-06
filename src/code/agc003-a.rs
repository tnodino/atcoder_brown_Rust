// https://atcoder.jp/contests/agc003/tasks/agc003_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let flg1 = S.contains("N");
    let flg2 = S.contains("W");
    let flg3 = S.contains("S");
    let flg4 = S.contains("E");
    if (flg1 && !flg3) || (!flg1 && flg3) {
        println!("No");
    }
    else if (flg2 && !flg4) || (!flg2 && flg4) {
        println!("No");
    }
    else {
        println!("Yes");
    }
}