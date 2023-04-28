use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    match a.iter().combinations(3).any(|v| v.into_iter().sum::<usize>() == 1000) {
        true => println!("Yes"),
        _ => println!("No"),
    };
}
