use proconio::input;

fn main() {
    input! {
        k: i32,
        x: i32
    }

    let r = ((x - k + 1)..(x + k)).collect::<Vec<i32>>();

    println!(
        "{}",
        r.into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
