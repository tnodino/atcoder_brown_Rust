// https://atcoder.jp/contests/abc088/tasks/abc088_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        c: [[isize; 3]; 3],
    }
    let mut a = [0; 3];
    let mut b = [0; 3];
    for i in 0..3 {
        a[i] = c[i][0];
    }
    for i in 0..3 {
        b[i] = c[0][i] - a[0];
    }
    for i in 0..3 {
        for j in 0..3 {
            if a[i] + b[j] != c[i][j] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}