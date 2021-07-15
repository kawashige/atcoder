use proconio::fastout;
use proconio::input;

fn dfs(s: &mut String, max: usize, n: usize) {
    if s.len() == n {
        println!("{}", s);
        return;
    }

    for i in 0..=max {
        s.push((b'a' + i as u8) as char);

        dfs(s, if i == max { max + 1 } else { max }, n);

        s.pop();
    }
}

#[fastout]
fn main() {
    input! {
        n: usize
    }

    dfs(&mut "a".to_string(), 1, n);
}
