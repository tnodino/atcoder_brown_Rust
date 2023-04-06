// https://atcoder.jp/contests/code-festival-2016-quala/tasks/codefestival_2016_qualA_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut ans = 0;
    for i in 0..N {
        if i < a[i] && a[a[i]-1] == i + 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}