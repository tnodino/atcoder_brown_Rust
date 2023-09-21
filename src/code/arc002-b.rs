// https://atcoder.jp/contests/arc002/tasks/arc002_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        i1: String,
    }
    let vec = i1.split("/").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let (mut Y, mut M, mut D) = (vec[0], vec[1], vec[2]);
    let mut day = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    while Y % (M * D) > 0 {
        if Y % 400 == 0 {
            day[1] = 29;
        }
        else if Y % 100 == 0 {
            day[1] = 28;
        }
        else if Y % 4 == 0 {
            day[1] = 29;
        }
        else {
            day[1] = 28;
        }
        if day[M-1] == D {
            D = 0;
            if M == 12 {
                M = 0;
                Y += 1;
            }
            M += 1;
        }
        D += 1;
    }
    println!("{:04}/{:02}/{:02}", Y, M, D);
}