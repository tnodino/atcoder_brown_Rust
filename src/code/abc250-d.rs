// https://atcoder.jp/contests/abc250/tasks/abc250_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = 1_000_000;
    let mut sieve = vec![true; M+1];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..=M {
        if sieve[i] {
            for j in (i..=M).step_by(i) {
                sieve[j] = false;
            }
            sieve[i] = true;
        }
    }
    let mut cnt = vec![0; M+1];
    for i in 2..=M {
        cnt[i] += cnt[i-1] + (sieve[i] as usize);
    }
    let mut ans = 0;
    for i in 2..=M {
        if i * i * i > N {
            break;
        }
        if !sieve[i] {
            continue;
        }
        let mut ok = 1;
        let mut ng = i;
        for _ in 0..100 {
            let mid = (ok + ng) / 2;
            if mid <= N / (i * i * i) {
                ok = mid;
            }
            else {
                ng = mid;
            }
        }
        ans += cnt[ok];
    }
    println!("{}", ans);
}