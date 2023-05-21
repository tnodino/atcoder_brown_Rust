// https://atcoder.jp/contests/arc156/tasks/arc156_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }
    for _ in 0..T {
        input! {
            N: usize,
            S: String,
        }
        let cnt = S.chars().filter(|&x| x == '1').count();
        if cnt == 0 {
            println!("0");
        }
        else if cnt % 2 == 1 {
            println!("-1");
        }
        else if cnt >= 4 {
            println!("{}", cnt / 2);
        }
        else if !S.contains("11") {
            println!("1");
        }
        else if N == 3 {
            println!("-1");
        }
        else if N == 4 && S == "0110".to_string() {
            println!("3");
        }
        else {
            println!("2");
        }
    }
}