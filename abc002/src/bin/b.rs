use proconio::input;

fn main() {
    input! {
        w: String
    }

    println!(
        "{}",
        w.chars()
            .filter(|c| !"aeiou".contains(&c.to_string()))
            .collect::<String>()
    );
}
