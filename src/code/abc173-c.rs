// https://atcoder.jp/contests/abc173/tasks/abc173_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        K: usize,
    }
    let mut c = Vec::new();
    for _ in 0..H {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        c.push(s);
    }
    let mut ans = 0;
    for bith in 0..1<<H {
        for bitw in 0..1<<W {
            let mut cnt = 0;
            let mut C = c.clone();
            for i in 0..H {
                if bith & (1 << i) > 0 {
                    for j in 0..W {
                        C[i][j] = '.';
                    }
                }
            }
            for j in 0..W {
                if bitw & (1 << j) > 0 {
                    for i in 0..H {
                        C[i][j] = '.';
                    }
                }
            }
            for i in 0..H {
                for j in 0..W {
                    if C[i][j] == '#' {
                        cnt += 1;
                    }
                }
            }
            if cnt == K {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}