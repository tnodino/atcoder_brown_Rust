// https://atcoder.jp/contests/agc007/tasks/agc007_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let mut A = Vec::new();
    for _ in 0..H {
        input! {
            a: String,
        }
        let a = a.chars().collect::<Vec<char>>();
        A.push(a);
    }
    let mut x = 0;
    let mut y = 0;
    A[x][y] = '.';
    while x + 1 < H || y + 1 < W {
        if x + 1 < H && A[x+1][y] == '#' {
            x += 1;
        }
        else if y + 1 < W && A[x][y+1] == '#' {
            y += 1;
        }
        else {
            println!("Impossible");
            return;
        }
        A[x][y] = '.';
    }
    for i in 0..H {
        for j in 0..W {
            if A[i][j] == '#' {
                println!("Impossible");
                return;
            }
        }
    }
    println!("Possible");
}