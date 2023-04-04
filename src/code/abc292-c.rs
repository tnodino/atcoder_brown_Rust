// https://atcoder.jp/contests/abc292/tasks/abc292_c

use proconio::input;
use proconio::fastout;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut cnt = vec![0; N+1];
    for i in 1..=N {
        let M = sqrt(i as f64) as usize;
        for m in 1..=M {
            if i % m == 0 {
                cnt[i] += 1;
                if i / m != m {
                    cnt[i] += 1;
                }
            }
        }
    }
    let mut ans: usize = 0;
    for x in 1..=N {
        let y = N - x;
        ans += cnt[x] * cnt[y];
    }
    println!("{}", ans);
}