// https://atcoder.jp/contests/abc257/tasks/abc257_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
        W: [usize; N],
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut vec = Vec::new();
    let mut ans = 0;
    for i in 0..N {
        vec.push((W[i], S[i]));
        if S[i] == '1' {
            ans += 1;
        }
    }
    vec.sort();
    let mut res = ans;
    for i in 0..N {
        match vec[i].1 {
            '0' => res += 1,
            _ => res -= 1,
        }
        if i + 1 < N {
            if vec[i].0 != vec[i+1].0 {
                ans = max(ans, res);
            }
        }
        else {
            ans = max(ans, res);
        }
    }
    println!("{}", ans);
}