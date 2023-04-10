// https://atcoder.jp/contests/abc258/tasks/abc258_b

use proconio::input;
use proconio::fastout;
use std::cmp::max;

const DX: [usize; 8] = [!0, 0, 1, !0, 1, !0, 0, 1];
const DY: [usize; 8] = [!0, !0, !0, 0, 0, 1, 1, 1];

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut A = Vec::new();
    for _ in 0..N {
        input! {
            mut a: usize,
        }
        let mut vec = Vec::new();
        for _ in 0..N {
            vec.push(a % 10);
            a /= 10;
        }
        A.push(vec);
    }
    let mut ans = 0;
    for i in 0..N {
        for j in 0..N {
            for d in 0..8 {
                let mut res = 0;
                let mut x = i;
                let mut y = j;
                for _ in 0..N {
                    res *= 10;
                    res += A[x][y];
                    let mut nx = x.wrapping_add(DX[d]);
                    let mut ny = y.wrapping_add(DY[d]);
                    if N <= nx {
                        if x == 0 {
                            nx = N - 1;
                        }
                        else {
                            nx = 0;
                        }
                    }
                    if N <= ny {
                        if y == 0 {
                            ny = N - 1;
                        }
                        else {
                            ny = 0;
                        }
                    }
                    x = nx;
                    y = ny;
                }
                ans = max(ans, res);
            }
        }
    }
    println!("{}", ans);
}