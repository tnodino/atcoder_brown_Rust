// https://atcoder.jp/contests/abc085/tasks/abc085_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut Y: usize,
    }
    Y /= 1000;
    for x in 0..=N {
        for y in 0..=N {
            if x + y > N {
                break;
            }
            let z = N - (x + y);
            if 10 * x + 5 * y + z == Y {
                println!("{} {} {}", x, y, z);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}