// https://atcoder.jp/contests/abc275/tasks/abc275_c

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let N: usize = 9;
    let M: isize = 9;
    let mut S = Vec::new();
    for _ in 0..N {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        S.push(s);
    }
    let mut pawn = HashSet::new();
    for i in 0..N {
        for j in 0..N {
            if S[i][j] == '#' {
                pawn.insert((i as isize, j as isize));
            }
        }
    }
    let mut ans = 0;
    for i in 0..N {
        for j in 0..N {
            if S[i][j] == '.' {
                continue;
            }
            let x1 = i as isize;
            let y1 = j as isize;
            for a in -M..M {
                for b in -M..M {
                    if a == 0 && b == 0 {
                        continue;
                    }
                    let x2 = x1 + a;
                    let y2 = y1 + b;
                    if !pawn.contains(&(x2, y2)) {
                        continue;
                    }
                    let x3 = x2 + b;
                    let y3 = y2 - a;
                    if !pawn.contains(&(x3, y3)) {
                        continue;
                    }
                    let x4 = x3 - a;
                    let y4 = y3 - b;
                    if !pawn.contains(&(x4, y4)) {
                        continue;
                    }
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans / 4);
}