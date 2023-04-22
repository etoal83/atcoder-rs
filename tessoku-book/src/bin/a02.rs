use proconio::input;

fn main() {
    input! {
	n: usize,
        x: usize,
        nums: [usize; n],
    }

    match nums.iter().any(|&i| i == x) {
        true => println!("Yes"),
        false => println!("No"),
    };
}
