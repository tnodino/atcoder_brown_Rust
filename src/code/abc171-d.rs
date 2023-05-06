// https://atcoder.jp/contests/abc171/tasks/abc171_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        Q: usize,
    }
    let M = 100_000;
    let mut cnt = vec![0; M+1];
    for i in 0..N {
        cnt[A[i]] += 1;
    }
    let mut ans = A.iter().sum::<usize>();
    for _ in 0..Q {
        input! {
            B: usize,
            C: usize,
        }
        ans -= cnt[B] * B;
        ans += cnt[B] * C;
        cnt[C] += cnt[B];
        cnt[B] = 0;
        println!("{}", ans);
    }
}