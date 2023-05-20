// https://atcoder.jp/contests/abc299/tasks/abc299_d

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    let stdin = std::io::stdin();
    let mut source = proconio::source::line::LineSource::new(stdin.lock());
    input! {
        from &mut source,
        N: usize,
    }
    let mut l = 1;
    let mut r = N;
    for _ in 0..20 {
        let mid = (l + r) / 2;
        println!("? {}", mid);
        input! {
            from &mut source,
            S: usize,
        }
        if S == 0 {
            l = mid;
        }
        else {
            r = mid;
        }
    }
    println!("! {}", l);
}