// https://atcoder.jp/contests/abc023/tasks/abc023_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut T = "b".chars().collect::<Vec<char>>();
    for i in 0..50 {
        if S == T {
            println!("{}", i);
            return;
        }
        let mut vec = Vec::new();
        if (i + 1) % 3 == 1 {
            vec.push('a');
            vec.extend(T);
            vec.push('c');
        }
        else if (i + 1) % 3 == 2 {
            vec.push('c');
            vec.extend(T);
            vec.push('a');
        }
        else {
            vec.push('b');
            vec.extend(T);
            vec.push('b');
        }
        T = vec;
    }
    println!("-1");
}