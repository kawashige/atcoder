use proconio::input;

fn main() {
    input! {
        s: String
    }

    print!("{}", s.replace(",", " "));
}
