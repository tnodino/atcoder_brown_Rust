// https://atcoder.jp/contests/tenka1-2015-qualb/tasks/tenka1_2015_qualB_a

use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let mut DP = [0; 20];
    DP[0] = 100;
    DP[1] = 100;
    DP[2] = 200;
    for i in 3..20 {
        DP[i] = DP[i-3] + DP[i-2] + DP[i-1];
    }
    println!("{}", DP[19]);
}