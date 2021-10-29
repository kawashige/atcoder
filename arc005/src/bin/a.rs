use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String; n]
    }

    let c = w
        .into_iter()
        .filter(|s| {
            let s = s.trim_end_matches('.');
            s == "TAKAHASHIKUN" || s == "Takahashikun" || s == "takahashikun"
        })
        .count();

    println!("{}", c);
}
