// https://atcoder.jp/contests/abc089/tasks/abc089_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut cnt = [0; 6];
    for _ in 0..N {
        input! {
            S: String,
        }
        let S = S.chars().collect::<Vec<char>>();
        cnt[match S[0] {
            'M' => 0,
            'A' => 1,
            'R' => 2,
            'C' => 3,
            'H' => 4,
            _ => 5,
        }] += 1;
    }
    let mut ans: usize = 0;
    for i in 0..5 {
        for j in i+1..5 {
            for k in j+1..5 {
                ans += cnt[i] * cnt[j] * cnt[k];
            }
        }
    }
    println!("{}", ans);
}