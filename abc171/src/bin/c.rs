use proconio::input;

fn main() {
    input! {
        n: u128
    }

    let mut chars = Vec::new();

    let mut remains = n;
    while remains > 0 {
        let i = (1..27).find(|num| (remains - num) % 26 == 0).unwrap();
        chars.push(((i - 1) as u8 + 0x61) as char);
        remains = (remains - i) / 26;
    }

    println!("{}", chars.iter().rev().collect::<String>());
}
