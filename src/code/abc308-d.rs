// https://atcoder.jp/contests/abc308/tasks/abc308_d

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

const DX: &[usize] = &[!0, 1, 0, 0];
const DY: &[usize] = &[0, 0, !0, 1];

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let mut S = Vec::new();
    for _ in 0..H {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        S.push(s);
    }
    if S[0][0] != 's' {
        println!("No");
        return;
    }
    let C = "snuke".chars().collect::<Vec<char>>();
    let mut flg = vec![vec![false; W]; H];
    flg[0][0] = true;
    let mut que: VecDeque<(usize, usize, usize)> = VecDeque::new();
    que.push_back((0, 0, 1));
    while !que.is_empty() {
        let (x, y, idx) = que.pop_front().unwrap();
        for d in 0..4 {
            let nx = x.wrapping_add(DX[d]);
            let ny = y.wrapping_add(DY[d]);
            if H <= nx || W <= ny {
                continue;
            }
            if flg[nx][ny] {
                continue;
            }
            if S[nx][ny] != C[idx] {
                continue;
            }
            flg[nx][ny] = true;
            que.push_back((nx, ny, (idx + 1) % 5));
        }
    }
    if flg[H-1][W-1] {
        println!("Yes");
    }
    else {
        println!("No");
    }
}