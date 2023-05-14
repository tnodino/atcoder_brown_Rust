// https://atcoder.jp/contests/arc104/tasks/arc104_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut ans = 0;
    for l in 0..N {
        let mut at = 0;
        let mut cg = 0;
        for r in l..N {
            match S[r] {
                'A' => at += 1,
                'T' => at -= 1,
                'C' => cg += 1,
                _ => cg -= 1,
            }
            if at == 0 && cg == 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}