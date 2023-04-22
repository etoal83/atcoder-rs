use proconio::input;

fn main() {
    input!{
        a: u32,
        b: u32,
    };

    match (a..=b).into_iter().any(|i| 100 % i == 0) {
        true => println!("Yes"),
        _ => println!("No"),
    };
}
