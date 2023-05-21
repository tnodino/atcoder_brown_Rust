// https://atcoder.jp/contests/abc287/tasks/abc287_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        T: String,
    }
    let N = T.len();
    let mut S = S.chars().collect::<Vec<char>>();
    let mut T = T.chars().collect::<Vec<char>>();
    let mut flg1 = vec![false; N+1];
    flg1[0] = true;
    for i in 0..N {
        if S[i] == T[i] || S[i] == '?' || T[i] == '?' {
            flg1[i+1] = true;
        }
        else {
            break;
        }
    }
    S.reverse();
    T.reverse();
    let mut flg2 = vec![false; N+1];
    flg2[0] = true;
    for i in 0..N {
        if S[i] == T[i] || S[i] == '?' || T[i] == '?' {
            flg2[i+1] = true;
        }
        else {
            break;
        }
    }
    for i in 0..=N {
        if flg1[i] && flg2[N-i] {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}