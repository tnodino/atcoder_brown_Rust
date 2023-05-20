// https://atcoder.jp/contests/arc110/tasks/arc110_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        T: String,
    }
    let M: usize = 10_000_000_000;
    let A = "110".repeat(N).to_string();
    let B = "101".repeat(N).to_string();
    let C = "011".repeat(N).to_string();
    if !A.contains(&T) && !B.contains(&T) && !C.contains(&T) {
        println!("0");
        return;
    }
    if T == "1".to_string() {
        println!("{}", M * 2);
    }
    else if T == "11".to_string() {
        println!("{}", M);
    }
    else {
        let zero = T.chars().filter(|&x| x == '0').count();
        if T.chars().last().unwrap() == '0' {
            println!("{}", M - zero + 1);
        }
        else {
            println!("{}", M - zero);
        }
    }
}