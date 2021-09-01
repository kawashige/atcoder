use proconio::input;

fn main() {
    input! {
        m: usize
    }

    let vv = match m {
        (0..=99) => "00".to_string(),
        (100..=5000) => format!("{:<02}", m / 100),
        (6000..=30000) => format!("{}", m / 1000 + 50),
        (35000..=70000) => format!("{}", (m / 1000 - 30) / 5 + 80),
        _ => "89".to_string(),
    };

    println!("{}", vv);
}
