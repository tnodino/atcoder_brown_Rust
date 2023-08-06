// https://atcoder.jp/contests/abc303/tasks/abc303_c

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        mut H: usize,
        K: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut set = HashSet::new();
    for _ in 0..M {
        input! {
            x: isize,
            y: isize,
        }
        set.insert((x, y));
    }
    let mut x = 0;
    let mut y = 0;
    for i in 0..N {
        if H == 0 {
            println!("No");
            return;
        }
        H -= 1;
        match S[i] {
            'R' => x += 1,
            'L' => x -= 1,
            'U' => y += 1,
            _ => y -= 1,
        };
        if H < K && set.contains(&(x, y)) {
            set.remove(&(x, y));
            H = K;
        }
    }
    println!("Yes");
}