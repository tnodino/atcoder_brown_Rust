// https://atcoder.jp/contests/abc201/tasks/abc201_c

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut ans = 0;
    'outer: for num in 0..=9999 {
        let mut flg = [false; 10];
        let mut x = num;
        for _ in 0..4 {
            flg[x%10] = true;
            x /= 10;
        }
        for i in 0..10 {
            if S[i] == 'o' && !flg[i] {
                continue 'outer;
            }
            if S[i] == 'x' && flg[i] {
                continue 'outer;
            }
        }
        ans += 1;
    }
    println!("{}", ans);
}