use proconio::input;

fn main() {
    input! {
        s: String
    }

    let count = s.chars().fold([0; 6], |mut count, c| {
        count[c as usize - b'A' as usize] += 1;
        count
    });

    println!(
        "{}",
        count
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
