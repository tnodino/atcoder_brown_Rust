// https://atcoder.jp/contests/diverta2019/tasks/diverta2019_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        R: usize,
        G: usize,
        B: usize,
        N: usize,
    }
    let mut ans = 0;
    for x in 0..=N {
        for y in 0..=N {
            if R * x + G * y > N {
                break;
            }
            let M = N - (R * x + G * y);
            if M % B == 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}