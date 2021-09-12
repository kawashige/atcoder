use proconio::input;

fn main() {
    input! {
        p: [u8; 26]
    }

    println!(
        "{}",
        p.into_iter()
            .map(|i| (i - 1 + 0x61) as char)
            .collect::<String>()
    );
}
