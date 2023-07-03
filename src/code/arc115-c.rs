// https://atcoder.jp/contests/arc115/tasks/arc115_c

use proconio::input;
use proconio::fastout;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut A = Vec::new();
    for i in 1..=N {
        let x = sqrt(i as f64) as usize + 1;
        let mut p = i;
        let mut cnt = 0;
        for k in 2..=x {
            while p % k == 0 {
                cnt += 1;
                p /= k;
            }
        }
        if p > 1 {
            cnt += 1;
        }
        A.push(cnt + 1);
    }
    println!("{}", A.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}