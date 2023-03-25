// https://atcoder.jp/contests/abc003/tasks/abc003_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        T: String,
    }
    let N = S.len();
    let S = S.chars().collect::<Vec<char>>();
    let T = T.chars().collect::<Vec<char>>();
    let mut ans = "You can win";
    for i in 0..N {
        if S[i] == T[i] {
            continue;
        }
        else {
            if S[i] == '@' {
                if !"atcoder".contains(T[i]) {
                    ans = "You will lose"
                }
            }
            else if T[i] == '@' {
                if !"atcoder".contains(S[i]) {
                    ans = "You will lose"
                }
            }
            else {
                ans = "You will lose"
            }
        }
    }
    println!("{}", ans);
}