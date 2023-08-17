// https://atcoder.jp/contests/abc240/tasks/abc240_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut stc = Vec::new();
    let mut ans = 0;
    for i in 0..N {
        ans += 1;
        if stc.is_empty() {
            stc.push((a[i], 1));
        }
        else {
            let (x, cnt) = stc.pop().unwrap();
            if x == a[i] {
                stc.push((x, cnt + 1));
            }
            else {
                stc.push((x, cnt));
                stc.push((a[i], 1));
            }
        }
        while !stc.is_empty() {
            let (x, cnt) = stc.pop().unwrap();
            if x > cnt {
                stc.push((x, cnt));
                break;
            }
            ans -= cnt;
        }
        println!("{}", ans);
    }
}