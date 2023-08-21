// https://atcoder.jp/contests/abc266/tasks/abc266_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Ax: isize,
        Ay: isize,
        Bx: isize,
        By: isize,
        Cx: isize,
        Cy: isize,
        Dx: isize,
        Dy: isize,
    }
    let x = vec![Ax, Bx, Cx, Dx];
    let y = vec![Ay, By, Cy, Dy];
    for i in 0..4 {
        let j = (i + 1) % 4;
        let k = (i + 2) % 4;
        let Px = x[k] - x[j];
        let Py = y[k] - y[j];
        let Qx = x[i] - x[j];
        let Qy = y[i] - y[j];
        if Px * Qy - Py * Qx <= 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}