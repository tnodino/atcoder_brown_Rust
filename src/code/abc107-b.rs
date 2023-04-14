// https://atcoder.jp/contests/abc107/tasks/abc107_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let mut a = Vec::new();
    for _ in 0..H {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        if s.contains(&'#') {
            a.push(s);
        }
    }
    let H = a.len();
    let mut flg = vec![vec![true; W]; H];
    for j in 0..W {
        let mut flg2 = false;
        for i in 0..H {
            if a[i][j] == '#' {
                flg2 = true;
            }
        }
        for i in 0..H {
            flg[i][j] = flg2;
        }
    }
    for i in 0..H {
        for j in 0..W {
            if flg[i][j] {
                print!("{}", a[i][j]);
            }
        }
        println!();
    }
}