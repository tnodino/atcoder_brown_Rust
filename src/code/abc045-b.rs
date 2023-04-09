// https://atcoder.jp/contests/abc045/tasks/abc045_b

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        SA: String,
        SB: String,
        SC: String,
    }
    let A = SA.len();
    let B = SB.len();
    let C = SC.len();
    let SA = SA.chars().collect::<Vec<char>>();
    let SB = SB.chars().collect::<Vec<char>>();
    let SC = SC.chars().collect::<Vec<char>>();
    let ma = [A, B, C];
    let S = [SA, SB, SC];
    let mut idx = [0, 0, 0];
    let mut now = 0;
    loop {
        if idx[now] == ma[now] {
            println!("{}", match now {
                0 => 'A',
                1 => 'B',
                _ => 'C',
            });
            return;
        }
        let c = S[now][idx[now]];
        idx[now] += 1;
        now = match c {
            'a' => 0,
            'b' => 1,
            _ => 2,
        }
    }

}