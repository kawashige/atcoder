use proconio::input;

fn main() {
    input! {
        s: String
    }

    if &s[5..7] <= "04" {
        println!("Heisei")
    } else {
        println!("TBD")
    }
}
