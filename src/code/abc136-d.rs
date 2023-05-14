// https://atcoder.jp/contests/abc136/tasks/abc136_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    let S = S.chars().collect::<Vec<char>>();
    let mut ans = vec![0; N];
    let mut cnt = 0;
    for i in 0..N {
        if S[i] == 'L' {
            ans[i] += cnt / 2;
            ans[i-1] += (cnt + 1) / 2;
            cnt = 0;
        }
        else {
            cnt += 1;
        }
    }
    cnt = 0;
    for i in (0..N).rev() {
        if S[i] == 'R' {
            ans[i] += cnt / 2;
            ans[i+1] += (cnt + 1) / 2;
            cnt = 0;
        }
        else {
            cnt += 1;
        }
    }
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}