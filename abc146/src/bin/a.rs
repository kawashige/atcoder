use proconio::input;

fn main() {
    input! {
        s: String
    }

    let v = ["SAT", "FRI", "THU", "WED", "TUE", "MON", "SUN"];
    let i = (0..7).find(|i| v[*i] == s).unwrap();

    println!("{}", i + 1);
}
