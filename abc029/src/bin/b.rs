use proconio::input;

fn main() {
    input! {
        s: [String; 12]
    }

    println!("{}", s.into_iter().filter(|x| x.contains('r')).count());
}
