// https://atcoder.jp/contests/abc096/tasks/abc096_c

use proconio::input;
use proconio::fastout;

const DX: [usize; 4] = [!0, 1, 0, 0];
const DY: [usize; 4] = [0, 0, !0, 1];

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let mut s = Vec::new();
    for _ in 0..H {
        input! {
            v: String,
        }
        let v = v.chars().collect::<Vec<char>>();
        s.push(v);
    }
    for i in 0..H {
        for j in 0..W {
            if s[i][j] == '.' {
                continue;
            }
            let mut flg = true;
            for d in 0..4 {
                let nx = i.wrapping_add(DX[d]);
                let ny = j.wrapping_add(DY[d]);
                if H <= nx || W <= ny {
                    continue;
                }
                if s[nx][ny] == '#' {
                    flg = false;
                }
            }
            if flg {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}