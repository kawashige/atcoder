use proconio::input;

fn main() {
    input! {
        s: String
    }

    let up = s[..2].parse::<usize>().unwrap();
    let down = s[2..].parse::<usize>().unwrap();

    if (1..13).contains(&up) && (1..13).contains(&down) {
        println!("AMBIGUOUS");
    } else if (1..13).contains(&up) {
        println!("MMYY");
    } else if (1..13).contains(&down) {
        println!("YYMM");
    } else {
        println!("NA");
    }
}
