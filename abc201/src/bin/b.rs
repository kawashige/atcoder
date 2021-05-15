use proconio::input;

fn main() {
    input! {
        n: usize,
        mut st: [(String, usize); n]
    }

    st.sort_unstable_by(|a, b| a.1.cmp(&b.1));

    println!("{}", st[n - 2].0);
}
