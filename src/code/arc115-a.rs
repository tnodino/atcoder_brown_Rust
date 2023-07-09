// https://atcoder.jp/contests/arc115/tasks/arc115_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        _M: usize,
    }
    let mut cnt: [usize; 2] = [0, 0];
    for _ in 0..N {
        input! {
            S: String,
        }
        cnt[S.chars().filter(|&x| x == '0').count() % 2] += 1;
    }
    println!("{}", cnt[0] * cnt[1]);
}