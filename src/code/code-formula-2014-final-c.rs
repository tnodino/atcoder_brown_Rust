// https://atcoder.jp/contests/code-formula-2014-final/tasks/code_formula_2014_final_c

use proconio::input;
use proconio::is_stdin_empty;

#[allow(non_snake_case)]
fn main() {
    let mut ans = Vec::new();
    loop {
        if is_stdin_empty() {
            break;
        }
        input! {
            S: String,
        }
        let mut flg = false;
        let mut user = "".to_string();
        for s in S.chars() {
            if s == '@' {
                if flg && user != "".to_string() {
                    ans.push(user);
                }
                flg = true;
                user = "".to_string();
            }
            else {
                user = format!("{}{}", user, s);
            }
        }
        if flg && user != "".to_string() {
            ans.push(user);
        }
    }
    ans.sort();
    ans.dedup();
    for a in ans {
        println!("{}", a);
    }
}