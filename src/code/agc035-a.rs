// https://atcoder.jp/contests/agc035/tasks/agc035_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut a: [usize; N],
    }
    a.sort();
    let mut b = a.clone();
    b.dedup();
    if b.len() >= 4 {
        println!("No");
    }
    else if b.len() == 1 {
        if b[0] == 0 {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
    else if N % 3 != 0 {
        println!("No");
    }
    else if b.len() == 2 {
        if b[0] == 0 {
            for i in 0..N {
                if a[i] != 0 {
                    if i * 2 == N - i {
                        println!("Yes");
                    }
                    else {
                        println!("No");
                    }
                    return;
                }
            }
        }
        else {
            println!("No");
        }
    }
    else {
        if b[0] ^ b[1] == b[2] {
            let n = N / 3;
            for i in 0..3 {
                if n != a.iter().filter(|&x| x == &b[i]).count() {
                    println!("No");
                    return;
                }
            }
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}