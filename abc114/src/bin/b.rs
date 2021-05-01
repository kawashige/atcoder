use proconio::input;

fn main() {
    input! {
        s: String
    }

    let mut min = std::i32::MAX;
    for i in 0..(s.len() - 2) {
        let n = (s[i..(i + 3)].parse::<i32>().unwrap() - 753).abs();
        min = std::cmp::min(min, n);
    }

    println!("{}", min)
}
