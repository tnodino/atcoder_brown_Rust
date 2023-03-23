// https://atcoder.jp/contests/arc002/tasks/arc002_2

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        YMD: String,
    }
    let vec = YMD.split("/").collect::<Vec<_>>();
    let mut Y = vec[0].parse::<usize>().unwrap();
    let mut M = vec[1].parse::<usize>().unwrap();
    let mut D = vec[2].parse::<usize>().unwrap();
    let mut day = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    loop {
        if Y % (M * D) == 0 {
            println!("{:04}/{:02}/{:02}", Y, M, D);
            return;
        }
        day[1] = 28;
        if Y % 4 == 0 {
            day[1] = 29;
        }
        if Y % 100 == 0 {
            day[1] = 28;
        }
        if Y % 400 == 0 {
            day[1] = 29;
        }
        D += 1;
        if day[M-1] + 1 == D {
            D = 1;
            M += 1;
        }
        if M == 13 {
            M = 1;
            Y += 1;
        }
    }
}