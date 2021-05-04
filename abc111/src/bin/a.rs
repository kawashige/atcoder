use proconio::input;

fn main() {
    input! {
        n: String
    }

    let s = n
        .chars()
        .map(|c| if c == '1' { '9' } else { '1' })
        .collect::<String>();

    println!("{}", s);
}
