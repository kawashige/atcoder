use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let mut count = [0; 4];
    for str in s {
        match str.as_str() {
            "AC" => count[0] += 1,
            "WA" => count[1] += 1,
            "TLE" => count[2] += 1,
            "RE" => count[3] += 1,
            _ => unreachable!(),
        }
    }

    println!("AC x {}", count[0]);
    println!("WA x {}", count[1]);
    println!("TLE x {}", count[2]);
    println!("RE x {}", count[3]);
}
