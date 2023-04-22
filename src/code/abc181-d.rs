// https://atcoder.jp/contests/abc181/tasks/abc181_d

use proconio::input;
use num::pow;

#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut cnt = [0; 10];
    for s in S.chars() {
        cnt[s as usize - 48] += 1;
    }
    let M = match S.len() {
        1 => 1,
        2 => 2,
        _ => 3,
    };
    let N = pow(10, M);
    'outer: for i in (0..N).step_by(8) {
        let T = i.to_string();
        let mut cnt2 = [0; 10];
        cnt2[0] += M - T.len();
        for t in T.chars() {
            cnt2[t as usize - 48] += 1;
        }
        for i in 0..10 {
            if cnt[i] < cnt2[i] {
                continue 'outer;
            }
        }
        println!("Yes");
        return;
    }
    println!("No");
}