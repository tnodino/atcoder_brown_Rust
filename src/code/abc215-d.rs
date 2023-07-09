// https://atcoder.jp/contests/abc215/tasks/abc215_d

use proconio::input;
use proconio::fastout;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N],
    }
    let mut eratos = vec![true; M+1];
    eratos[0] = false;
    for i in 0..N {
        let L = sqrt(A[i] as f64) as usize + 1;
        let mut a = A[i];
        for l in 2..=L {
            if a % l == 0 && l <= M && eratos[l] {
                for j in (l..=M).step_by(l) {
                    eratos[j] = false;
                }
            }
            while a % l == 0 {
                a /= l;
            }
        }
        if a > 1 && a <= M && eratos[a] {
            for j in (a..=M).step_by(a) {
                eratos[j] = false;
            }
        }
    }
    println!("{}", eratos.iter().filter(|&x| *x).count());
    for i in 1..=M {
        if eratos[i] {
            println!("{}", i);
        }
    }
}