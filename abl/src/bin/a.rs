use proconio::input;

fn main() {
    input! {
        k: usize
    }

    println!(
        "{}",
        std::iter::repeat("ACL".to_string())
            .take(k)
            .collect::<String>()
    );
}
