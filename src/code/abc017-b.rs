// https://atcoder.jp/contests/abc017/tasks/abc017_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: String,
    }
    let N = X.len();
    let X = X.chars().collect::<Vec<char>>();
    let mut idx = 0;
    while idx < N {
        if X[idx] == 'o' || X[idx] == 'k' || X[idx] == 'u' {
            idx += 1;
            continue;
        }
        if idx + 1 < N && X[idx] == 'c' && X[idx+1] == 'h' {
            idx += 2;
            continue;
        }
        println!("NO");
        return;
    }
    println!("YES");
}