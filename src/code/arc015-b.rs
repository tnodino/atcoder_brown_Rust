// https://atcoder.jp/contests/arc015/tasks/arc015_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut cnt = [0; 6];
    for _ in 0..N {
        input! {
            MT: f64,
            mT: f64,
        }
        if 35. <= MT {
            cnt[0] += 1;
        }
        if 30. <= MT && MT < 35. {
            cnt[1] += 1;
        }
        if 25. <= MT && MT < 30. {
            cnt[2] += 1;
        }
        if 25. <= mT {
            cnt[3] += 1;
        }
        if mT < 0. && 0. <= MT {
            cnt[4] += 1;
        }
        if MT < 0. {
            cnt[5] += 1;
        }
    }
    println!("{}", cnt.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}