// https://atcoder.jp/contests/arc016/tasks/arc016_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut flg = [false; 9];
    let mut ans = 0;
    for _ in 0..N {
        input! {
            x: String,
        }
        for (i, v) in x.chars().enumerate() {
            if v == 'o' {
                if flg[i] {
                    continue;
                }
                flg[i] = true;
                ans += 1;
            }
            else {
                flg[i] = false;
                if v == 'x' {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}