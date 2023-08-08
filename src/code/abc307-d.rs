// https://atcoder.jp/contests/abc307/tasks/abc307_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut stk = Vec::new();
    let mut idx = 0;
    let mut cnt = 0;
    for i in 0..N {
        stk.push(S[i]);
        idx += 1;
        if S[i] == '(' {
            cnt += 1;
        }
        else if S[i] == ')' && cnt > 0 {
            cnt -= 1;
            while idx > 0 {
                let tmp = stk.pop().unwrap();
                idx -= 1;
                if tmp == '(' {
                    break;
                }
            }
        }
    }
    println!("{}", stk.iter().collect::<String>());
}