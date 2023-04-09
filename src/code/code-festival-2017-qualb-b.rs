// https://atcoder.jp/contests/code-festival-2017-qualb/tasks/code_festival_2017_qualb_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut D: [usize; N],
        M: usize,
        mut T: [usize; M],
    }
    D.sort();
    T.sort();
    let mut idx = 0;
    for i in 0..N {
        if D[i] == T[idx] {
            idx += 1;
            if idx == M {
                println!("YES");
                return;
            }
        }
    }
    println!("NO");
}