// https://atcoder.jp/contests/agc032/tasks/agc032_a

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut b: [usize; N],
    }
    let mut vec = Vec::new();
    'outer: for _ in 0..N {
        for i in (0..b.len()).rev() {
            if b[i] == i + 1 {
                vec.push(i + 1);
                b.remove(i);
                continue 'outer;
            }
        }
        println!("-1");
        return;
    }
    vec.reverse();
    for v in vec {
        println!("{}", v);
    }
}