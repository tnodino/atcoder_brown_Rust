// https://atcoder.jp/contests/abc069/tasks/arc080_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut cnt1 = 0;
    let mut cnt2 = 0;
    let mut cnt3 = 0;
    for i in 0..N {
        if a[i] % 4 == 0 {
            cnt3 += 1;
        }
        else if a[i] % 2 == 0 {
            cnt2 += 1;
        }
        else {
            cnt1 += 1;
        }
    }
    if cnt2 == 0 {
        if cnt1 <= cnt3 + 1 {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
    else {
        if cnt1 <= cnt3 {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}